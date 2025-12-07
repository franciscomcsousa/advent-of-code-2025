use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn get_adjacent(position: (i16, i16)) -> Vec<(i16, i16)> {
    let mut adjacents: Vec<(i16, i16)> = Vec::new();

    for x in -1..=1 {
        for y in -1..=1 {
            adjacents.push((position.0 + x, position.1 + y));
        }
    }

    adjacents
}

fn remove_paper(grid: &HashSet<(i16, i16)>) -> Vec<(i16, i16)> {
    let mut to_remove: Vec<(i16, i16)> = Vec::new();
    for &position in grid.iter() {
        let adjacents = get_adjacent(position);
        let mut papers: i8 = 0;
        for adjacent in adjacents {
            if grid.contains(&adjacent) {
                papers += 1;
            }
        }
        if !(papers >= 5) {
            to_remove.push(position);
        }
    }

    to_remove
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut total: usize = 0;
    let mut grid: HashSet<(i16, i16)> = HashSet::new();

    for (row, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        for (column, char) in line.chars().enumerate() {
            if char == '@' {
                grid.insert((row as i16, column as i16));
            }
        }
    }

    loop {
        let to_remove: Vec<(i16, i16)> = remove_paper(&grid);
        if to_remove.len() == 0 {
            break;
        }
        total += to_remove.len();
        for position in to_remove {
            grid.remove(&position);
        }
    }

    println!("Total: {}", total);

    Ok(())
}
