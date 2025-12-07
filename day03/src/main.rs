use std::io::{self, BufRead};

fn get_jolt(line: &str, size: i32) -> String {
    let mut highest_jolt = 0;
    let mut jolt_index = 0;
    if size == 0 {
        return "".to_string();
    }
    for (i, ch) in line
        .chars()
        .enumerate()
        .take(line.len() - (size - 1) as usize)
    {
        let num = ch.to_digit(10).unwrap() as usize;
        if num > highest_jolt {
            highest_jolt = num;
            jolt_index = i;
        }
    }
    let mut result = highest_jolt.to_string();
    result.push_str(&get_jolt(&line[jolt_index + 1..], size - 1));

    return result;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut total: usize = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let jolt = get_jolt(&line, 12);

        total += jolt.parse::<usize>().unwrap();
    }

    println!("{total}");

    Ok(())
}
