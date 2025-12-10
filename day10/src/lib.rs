use std::{error::Error, fs, num::ParseIntError, path::PathBuf, str::FromStr};

use itertools::Itertools;
use microlp::{ComparisonOp, Problem};

#[derive(Debug)]
struct MachineSpec {
    lights: u32,
    buttons: Vec<Vec<u32>>,
    joltages: Vec<i32>,
}

#[derive(Debug)]
struct ParseMachineSpecError;

impl MachineSpec {
    fn button_bitsets(&self) -> Vec<u32> {
        self.buttons
            .iter()
            .map(|b| b.iter().map(|v| 2_u32.pow(*v)).sum())
            .collect()
    }
}

impl FromStr for MachineSpec {
    type Err = ParseMachineSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut specs = s.split(" ").collect::<Vec<_>>();

        let jolt_spec = specs.pop().ok_or(ParseMachineSpecError)?;
        let light_spec = specs.first().ok_or(ParseMachineSpecError)?;

        let button_specs = specs.get(1..).ok_or(ParseMachineSpecError)?;

        let lights = light_spec
            .strip_prefix("[")
            .and_then(|s| s.strip_suffix("]"))
            .and_then(|s| Some(s.chars()))
            .and_then(|sp| {
                Some(
                    sp.map(|s| s == '#')
                        .enumerate()
                        .filter(|(_, s)| *s)
                        .map(|(i, _)| 2_u32.pow(i as u32))
                        .sum(),
                )
            })
            .ok_or(ParseMachineSpecError)?;

        let joltages = jolt_spec
            .strip_prefix("{")
            .and_then(|s| s.strip_suffix("}"))
            .and_then(|s| {
                Some(
                    s.split(",")
                        .map(|i| i.parse::<i32>())
                        .collect::<Result<Vec<_>, ParseIntError>>(),
                )
            })
            .and_then(|r| r.ok())
            .ok_or(ParseMachineSpecError)?;

        let buttons = button_specs
            .iter()
            .map(|s| {
                s.strip_prefix("(")
                    .and_then(|s| s.strip_suffix(")"))
                    .and_then(|s| {
                        Some(
                            s.split(",")
                                .map(|b| b.parse::<u32>())
                                .collect::<Result<Vec<_>, ParseIntError>>(),
                        )
                    })
                    .and_then(|r| r.ok())
                    .ok_or(ParseMachineSpecError)
            })
            .collect::<Result<Vec<_>, ParseMachineSpecError>>()?;

        Ok(MachineSpec {
            lights,
            buttons,
            joltages,
        })
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    let machine_specs = puzzle_input.lines().map(|line| {
        line.trim()
            .parse::<MachineSpec>()
            .expect("Failed to parse machine spec")
    });

    if part_two {
        machine_specs
            .map(|m| {
                // Set up and solve an integer linear programming problem with an exact constraint
                let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);

                let variables = (0..m.buttons.len())
                    .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
                    .collect::<Vec<_>>();

                for (i, joltage) in m.joltages.iter().enumerate() {
                    let vars = m
                        .buttons
                        .iter()
                        .enumerate()
                        .filter_map(|(j, b)| {
                            if b.contains(&(i as u32)) {
                                Some((variables[j], 1.0))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    problem.add_constraint(vars, ComparisonOp::Eq, *joltage as f64);
                }
                let solution = problem.solve().expect("Failed to solve ILP problem");
                solution.objective().round() as u64
            })
            .sum::<u64>()
    } else {
        machine_specs
            .map(|m| {
                (1..=m.buttons.len())
                    .find_map(|i| {
                        m.button_bitsets().iter().combinations(i).find_map(|c| {
                            let button_xor = c.iter().fold(0_u32, |acc, b| acc ^ *b);
                            if button_xor == m.lights {
                                Some(i as u64)
                            } else {
                                None
                            }
                        })
                    })
                    .expect("None of the button combinations matched the lights")
            })
            .sum()
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
