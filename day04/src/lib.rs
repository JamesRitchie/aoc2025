use std::{collections::HashSet, error::Error, fs, path::PathBuf};

use itertools::Itertools;

fn get_removable_rolls(roll_locations: &HashSet<(i32, i32)>) -> impl Iterator<Item = &(i32, i32)> {
    roll_locations.iter().filter(|(i, j)| {
        let neighbor_count = (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(di, dj)| !(di == &0 && dj == &0))
            .filter(|(di, dj)| roll_locations.contains(&(i + di, j + dj)))
            .count();
        neighbor_count < 4
    })
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let mut roll_locations = puzzle_input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.to_string() == "@")
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .flatten()
        .collect::<HashSet<_>>();

    let removable_rolls = get_removable_rolls(&roll_locations);

    if part_two {
        let mut removed_count = 0;

        let mut removable_rolls = removable_rolls.cloned().collect::<HashSet<_>>();

        while !removable_rolls.is_empty() {
            removed_count += removable_rolls.len();
            roll_locations = &roll_locations - &removable_rolls;
            removable_rolls = get_removable_rolls(&roll_locations).cloned().collect();
        }

        removed_count as i64
    } else {
        removable_rolls.count() as i64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
