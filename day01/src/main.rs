use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut original_counter = 0;
    let mut password_counter: i32 = 0;
    let mut current_value: i32 = 50;

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        let line = line_result?;
        let (direction, clicks) = line.split_at(1);
        let clicks: i32 = clicks.parse().unwrap();

        let mut rotate = |value: i32| {
            println!("Value: {}", value);
            if current_value != 0 && current_value + value <= 0 {
                password_counter += 1;
            }

            password_counter += (current_value + value).abs() / 100;

            current_value = (current_value + value).rem_euclid(100);
            if current_value == 0 {
                original_counter += 1;
            }
            println!(
                "Current Value: {}; OG: {} ; Password: {}",
                current_value, original_counter, password_counter
            );
        };

        match direction {
            "L" => rotate(-clicks),
            "R" => rotate(clicks),
            _ => (),
        }
    }
    println!("Part 1: {}", original_counter);
    println!("Part 2: {}", password_counter);

    Ok(())
}
