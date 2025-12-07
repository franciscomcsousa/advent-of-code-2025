use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ranges = Vec::new();
    let mut lines = reader.lines();
    let mut total = 0;

    while let Some(line_result) = lines.next() {
        let line = line_result?;
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.trim().split('-').collect();
        let (min, max) = (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        );
        ranges.push(min..=max);
    }

    for line_result in lines {
        let line = line_result?;
        let id = line.parse::<i64>().unwrap();

        for range in &ranges {
            if range.contains(&id) {
                total += 1;
                break;
            }
        }
    }

    println!("{total}");
    Ok(())
}
