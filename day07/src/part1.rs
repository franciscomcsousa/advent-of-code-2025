use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut counter = 0;
    let mut beams: Vec<HashSet<usize>> = Vec::new();

    for line_result in io::stdin().lock().lines() {
        let line = line_result?;
        beams.push(beams.last().cloned().unwrap_or_else(HashSet::new));

        for (index, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    beams.last_mut().unwrap().insert(index);
                    continue;
                }
                '^' => {
                    if beams.last().unwrap().contains(&index) {
                        counter += 1;
                        if index > 0 {
                            beams.last_mut().unwrap().insert(index - 1);
                        }
                        if index < line.len() {
                            beams.last_mut().unwrap().insert(index + 1);
                        }
                        beams.last_mut().unwrap().remove(&index);
                    }
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    println!("Counter: {counter}");

    Ok(())
}