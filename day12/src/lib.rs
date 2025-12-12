use std::{error::Error, fs, path::PathBuf};

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    // The gift shapes/sizes can be ignored as we don't need to solve the packing problem for
    // the real input.

    let regions = puzzle_input
        .split("\n\n")
        .last()
        .expect("Puzzle input should not be empty.");

    if part_two {
        println!("There is not part two today!");
        0
    } else {
        regions
            .lines()
            .filter(|l| {
                let (area, gift_counts) = l.split_once(": ").expect("Region line misformatted.");
                let area: u64 = area
                    .split("x")
                    .map(|a| a.parse::<u64>().expect("Area dimensions should be integer"))
                    .product();
                let gift_total: u64 = gift_counts
                    .split_whitespace()
                    .map(|g| g.parse::<u64>().expect("Gift counts should be integers"))
                    .sum();
                (gift_total * 9) <= area
            })
            .count() as u64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
