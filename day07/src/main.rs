use std::{
    collections::{HashMap},
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let mut beams: Vec<HashMap<usize, usize>> = Vec::new();

    for line_result in io::stdin().lock().lines() {
        let line = line_result?;
        beams.push(beams.last().cloned().unwrap_or_else(HashMap::new));

        for (index, char) in line.chars().enumerate() {
            let last_beams = beams.last_mut().unwrap();
            match char {
                'S' => {
                    last_beams.insert(index, 1);
                    continue;
                }
                '^' => {
                    if last_beams.contains_key(&index) {
                        let count = last_beams[&index];
                        match last_beams.get_mut(&(index - 1)) {
                            Some(value) => *value += count,
                            None => {
                                last_beams.insert(index - 1, count);
                            }
                        }

                        match last_beams.get_mut(&(index + 1)) {
                            Some(value) => *value += count,
                            None => {
                                last_beams.insert(index + 1, count);
                            }
                        }

                        last_beams.remove(&index);
                    }
                    continue;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    let mut timelines = 0;
    let highest_beams = beams.pop().unwrap();
    for (key, value) in highest_beams {
        timelines += value;
    }

    println!("{timelines}");

    Ok(())
}
