use std::io::{self, BufRead};

enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut results: Vec<i64> = Vec::new();

    let mut operations: Vec<Vec<String>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<String> = line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        operations.push(parts);
    }

    for column in 0..operations[0].len() {
        let operator = match operations[operations.len() - 1][column].as_str() {
            "*" => {
                results.push(1);
                Operator::Multiply
            }
            "+" => {
                results.push(0);
                Operator::Add
            }
            _ => panic!(
                "Invalid operator \"{}\" on column {}.",
                operations[operations.len() - 1][column],
                column
            ),
        };

        for line in 0..operations.len() - 1 {
            let value = operations[line][column].parse::<i64>().unwrap();
            results[column] = operator.apply(results[column], value);
        }
    }

    let total: i64 = results.iter().sum();

    println!("{total}");

    Ok(())
}
