use std::{io::{self, BufRead}, vec};

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

struct Operation {
    operator: Operator,
    numbers: Vec<i64>,
}

impl Operation {
    fn new(operator: Operator, numbers: Vec<i64>) -> Self {
        Operation { operator, numbers }
    }

    fn calculate(&self) -> i64 {
        let mut result: i64;
        match self.operator {
            Operator::Add => {
                result = 0;
            },
            Operator::Multiply => {
                result = 1;
            }
        }
        for number in self.numbers.iter() {
            result = self.operator.apply(result, *number);
        }
        result
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut results: Vec<i64> = Vec::new();

    let mut lines: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line.chars().rev().collect());
    }

    let mut numbers: Vec<String> = vec!["".to_string(); lines[0].len()];

    for column in 0..lines[0].len() {
        for line in 0..(lines.len() - 1){
            let char = lines[line].chars().nth(column).unwrap();
            if char == ' ' {
                continue;
            }
            else {
                numbers[column].push(char);
            }
        }
    }

    let mut indexes : Vec<usize> = Vec::new();
    for (i, char) in lines[lines.len() - 1].chars().enumerate() {
        if char == ' ' {
            indexes.push(i);
            continue;
        }
        indexes.push(i);

        let operator = match char {
            '*' => {
                Operator::Multiply
            }
            '+' => {
                Operator::Add
            }
            _ => panic!(
                "Invalid operator \"{}\" on column {}.", char, i),
        };

        let mut operation = Operation::new(operator, Vec::new());

        for index in indexes.iter() {
            if numbers[*index].is_empty() {
                continue;
            }
            let value = numbers[*index].parse::<i64>().unwrap();
            operation.numbers.push(value);
        }
        
        results.push(operation.calculate());
        indexes = Vec::new();
    }

    let total: i64 = results.iter().sum();

    println!("{total}");

    Ok(())
}
