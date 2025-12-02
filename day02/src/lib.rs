use itertools::Itertools;
use std::{error::Error, fs, path::PathBuf, vec};

fn compute_splits(digits: i64, part_two: bool) -> Vec<i64> {
    // Compute ways of splitting the number into equal parts
    if part_two {
        (1..=(digits / 2)).filter(|&d| digits % d == 0).collect()
    } else if digits % 2 == 0 {
        vec![digits / 2]
    } else {
        vec![]
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {

    puzzle_input
        .split(",")
        .map(|s| {
            let (start, stop) = s.split_once("-").expect("Range should have two parts");
            let start: i64 = start.parse().expect("Start should be an integer");
            let stop: i64 = stop.parse().expect("Stop should be an integer");

            (start..stop + 1)
                .filter(|&n| {
                    let digits = (n as f64).log10().floor() as i64 + 1;

                    let splits = compute_splits(digits, part_two);

                    splits
                        .iter()
                        .map(|s| {
                            let m = digits / s;
                            let r = 10_i64.pow(*s as u32);
                            (0..m)
                                .map(|i| {
                                    let denom = 10_i64.pow((i * s) as u32) as i64;
                                    (n / denom) % r
                                })
                                .all_equal()
                        })
                        .any(|x| x)
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
