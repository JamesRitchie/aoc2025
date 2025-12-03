use std::{error::Error, fs, path::PathBuf};

fn argmax(values: &[i64]) -> usize {
    let mut max_index = 0;
    let mut max_value = values[0];
    for (i, &value) in values.iter().enumerate().skip(1) {
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }
    max_index
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

    let total_digits;
    if part_two {
        total_digits = 12;
    } else {
        total_digits = 2;
    }

    let mut answer = 0;
    let mut max_index = 0;

    for remaining_digits in (0..total_digits).rev() {
        max_index = argmax(&digits[(max_index)..(digits_len - remaining_digits)]) + max_index;
        answer = (answer * 10) + digits[max_index];
        max_index += 1;
    }
    answer
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
