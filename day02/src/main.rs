use std::{collections::HashSet, io};

fn number(string: &str) -> usize {
    string.parse::<usize>().unwrap()
}

fn in_interval(s1: &str, start: usize, end: usize) -> bool {
    if number(s1) >= start && number(s1) <= end {
        return true;
    }
    false
}

fn get_all_possible(start: &str, end: &str) -> HashSet<usize> {
    let mut combinations: HashSet<usize> = HashSet::new();

    println!("Start: {start}, End: {end}");

    let mut i = 1;
    let mut j = 1;
    if number(&start[..i]) > number(&end[..j]) {
        combinations.extend(number(&end[..j])..number(&start[..i]) + 1);
        println!("Range {}..{} ", number(&start[..i]), number(&end[..j]));
        j = 2;
    }
    while i <= (start.len() + 1) / 2 && j <= (end.len() + 1) / 2 {
        combinations.extend(number(&start[..i])..number(&end[..j]) + 1);
        println!("Range {}..{} ", number(&start[..i]), number(&end[..j]));
        i += 1;
        j += 1;
    }

    combinations
}

fn assess_interval(part: &str) -> HashSet<usize> {
    let numbers: Vec<&str> = part.split('-').collect();

    let mut total: HashSet<usize> = HashSet::new();

    let (start, end): (&str, &str) = (numbers[0], numbers[1]);
    let (start_number, end_number): (usize, usize) = (number(start), number(end));
    let (start_size, end_size): (usize, usize) = (start.len(), end.len());

    let possible = get_all_possible(start, end);

    let mut candidates: HashSet<String> = HashSet::new();

    let mut create_repeating = |number: usize, target_size: usize| {
        if target_size % number.to_string().len() == 0 {
            let repeating = target_size / number.to_string().len();
            candidates.insert(number.to_string().repeat(repeating));
        }
    };
    for number in possible {
        create_repeating(number, end_size);
        create_repeating(number, start_size);
    }

    for candidate in candidates {
        if candidate.len() == 1 {
            continue;
        }
        if in_interval(&candidate, start_number, end_number) {
            total.insert(number(&candidate));
            println!("{}", &candidate);
        }
    }
    total
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let parts: Vec<&str> = line.trim().split(',').collect();

    let mut total: HashSet<usize> = HashSet::new();

    for part in parts {
        total.extend(assess_interval(part));
    }

    let sum: usize = total.into_iter().sum();
    println!("Total: {sum}")
}
