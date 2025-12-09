use std::{
    cmp::{max, min},
    error::Error,
    fs,
    path::PathBuf,
};

use itertools::Itertools;

struct Rectangle {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    area: i64,
}

impl Rectangle {
    fn new(a: (i64, i64), b: (i64, i64)) -> Rectangle {
        let x_min = min(a.0, b.0);
        let x_max = max(a.0, b.0);
        let y_min = min(a.1, b.1);
        let y_max = max(a.1, b.1);

        Rectangle {
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
            area: (x_max - x_min + 1) * (y_max - y_min + 1),
        }
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    let mut coords = puzzle_input
        .lines()
        .map(|l| {
            let (x, y) = l
                .split_once(",")
                .expect("Coordinates should be split by commas");
            (
                x.parse::<i64>().expect("X-coord should be integer"),
                y.parse::<i64>().expect("Y-coord should be integer"),
            )
        })
        .collect::<Vec<_>>();

    // Needed for wrap-around of tiles in part 2
    coords.push(coords[0]);

    let rectangles = coords
        .iter()
        .combinations(2)
        .map(|v| Rectangle::new(*v[0], *v[1]));

    // Line sections are just rectangles with one side of length 1
    let line_sections = coords
        .windows(2)
        .map(|v| Rectangle::new(v[0], v[1]))
        .collect::<Vec<_>>();

    if part_two {
        rectangles
            .filter(|r| {
                line_sections.iter().all(|l| {
                    // Filter for rectangles where every line section passes outside the 
                    // interior of the rectangle
                    l.x_max <= r.x_min
                        || l.x_min >= r.x_max
                        || l.y_max <= r.y_min
                        || l.y_min >= r.y_max
                })
            })
            .map(|r| r.area)
            .max()
            .expect("At least one rectangle should be free of line intersections") as u64
    } else {
        rectangles
            .map(|r| r.area)
            .max()
            .expect("Input should not be empty") as u64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
