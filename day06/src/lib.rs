use std::{error::Error, fs, iter::zip, path::PathBuf};

use itertools::Itertools;

pub fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..(matrix[0].len()))
        .map(|c| matrix.iter().map(|r| r[c].clone()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    let mut lines = puzzle_input.lines().rev();

    let ops = lines
        .next()
        .expect("Puzzle input should have at least one line")
        .split_whitespace()
        .collect::<Vec<_>>();

    let values_t;

    if part_two {
        // Parse the input into a grid of characters
        let grid = lines
            .rev()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // Transpose so columns come first
        let grid_transpose = transpose(&grid);

        // Split by columns of spaces and filter out the space columns
        let chunker = grid_transpose
            .iter()
            .chunk_by(|r| r.iter().all(|c| *c == ' '));
        let chunks = chunker.into_iter().filter(|(k, _)| !k).map(|(_, g)| g);

        values_t = chunks
            .map(|g| {
                g.map(|r| {
                    r.iter()
                        .collect::<String>()
                        .trim()
                        .parse::<u64>()
                        .expect("Line items should be ints")
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    } else {
        let values = lines
            .map(|l| {
                l.split_whitespace()
                    .map(|s| s.parse::<u64>().expect("Line items should be ints"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        values_t = transpose(&values);
    }

    zip(values_t, ops)
        .map(|(vs, o)| match o {
            "*" => vs.into_iter().product::<u64>(),
            "+" => vs.into_iter().sum::<u64>(),
            _ => panic!("Invalid op"),
        })
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
