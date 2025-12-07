use std::{
    collections::HashSet,
    io::{self, BufRead},
    ops::RangeInclusive,
};

fn highest_end(number: i64, ranges: &HashSet<RangeInclusive<i64>>) -> Option<i64> {
    let mut highest: Option<i64> = None;
    for range in ranges {
        if range.contains(&number) {
            match highest {
                Some(current_highest) => {
                    if *range.end() > current_highest {
                        highest = Some(*range.end());
                    }
                }
                None => highest = Some(*range.end()),
            }
        }
    }
    highest
}

fn lowest_start(number: i64, ranges: &HashSet<RangeInclusive<i64>>) -> Option<i64> {
    let mut lowest: Option<i64> = None;
    for range in ranges {
        if range.contains(&number) {
            match lowest {
                Some(current_lowest) => {
                    if *range.start() < current_lowest {
                        lowest = Some(*range.start());
                    }
                }
                None => lowest = Some(*range.start()),
            }
        }
    }
    lowest
}

fn normalize_range(
    ranges: &HashSet<RangeInclusive<i64>>,
    inserted_range: RangeInclusive<i64>,
) -> Vec<RangeInclusive<i64>> {
    let mut to_remove: Vec<RangeInclusive<i64>> = Vec::new();
    for range in ranges {
        if inserted_range.contains(range.start()) && inserted_range.contains(range.end()) {
            to_remove.push(*range.start()..=*range.end());
        }
    }
    to_remove
}

// Assumes vector of non overlaping ranges
fn count_ranges(ranges: &HashSet<RangeInclusive<i64>>) -> i64 {
    let mut total = 0;
    for range in ranges {
        println!("Start: {} End: {}", range.start(), range.end());
        total += range.end() - range.start() + 1;
    }
    total
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut ranges = HashSet::new();
    let mut lines = reader.lines();

    while let Some(line_result) = lines.next() {
        let line = line_result?;
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.trim().split('-').collect();
        let (mut min, mut max) = (
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
        );

        match highest_end(min, &ranges) {
            Some(value) => min = value + 1,
            None => (),
        };

        match lowest_start(max, &ranges) {
            Some(value) => max = value - 1,
            None => (),
        }

        if max >= min {
            let to_remove = normalize_range(&ranges, min..=max);
            for range in to_remove {
                ranges.remove(&range);
            }
            ranges.insert(min..=max);
        }
    }

    println!("{}", count_ranges(&ranges));
    Ok(())
}
