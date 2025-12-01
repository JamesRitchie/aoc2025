use std::{error::Error, fs, path::PathBuf};

fn parse_line(line: &str) -> i32 {
    // Parse the line to get the rotation value. Left turns are negative, right turns are positive.
    line.replace("L", "-").replace("R", "").parse().unwrap()
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i32 {
    let rotations = puzzle_input.lines().map(|line| parse_line(line));

    let (_, zeros, crossings) = rotations.fold((50, 0, 0), |(mut p, mut z, mut c), x| {
        c += (x / 100).abs();

        let end = (p + x).rem_euclid(100);
        if p != 0 && end != 0 {
            if p > end && x.signum() > 0 {
                c += 1;
            } else if p < end && x.signum() < 0 {
                c += 1;
            }
        }
        p = end;
        if p == 0 {
            z += 1;
        }
        (p, z, c)
    });
    if part_two { crossings + zeros } else { zeros }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i32, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
