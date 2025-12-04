use std::{error::Error, fs, path::PathBuf};

fn max_pos_value(values: &[i64]) -> (usize, i64) {
    // Find the position and value of the maximum element in values
    let max_value = values.iter().max().expect("values should not be empty");
    let idx = values
        .iter()
        .position(|&v| v == *max_value)
        .expect("max_value should be in values");
    (idx, *max_value)
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    puzzle_input
        .lines()
        .map(|line| process_line(line, part_two))
        .sum()
}

fn process_line(line: &str, part_two: bool) -> i64 {
    let digits = line
        .chars()
        .map(|c| {
            c.to_string()
                .parse::<i64>()
                .expect("Each character should be a digit.")
        })
        .collect::<Vec<_>>();

    let digits_len = digits.len();

    let total_digits = if part_two { 12 } else { 2 };

    let (answer, _) =
        (0..total_digits)
            .rev()
            .fold((0, 0), |(answer, mut max_index), remaining_digits| {
                let (idx, max_value) =
                    max_pos_value(&digits[(max_index)..(digits_len - remaining_digits)]);
                max_index += idx;
                (answer * 10 + max_value, max_index + 1)
            });
    answer
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
