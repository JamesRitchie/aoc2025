# Advent of Code 2025
![Build Status](https://github.com/JamesRitchie/aoc2025/actions/workflows/rust.yml/badge.svg)

My solutions to [Advent of Code 2025](https://adventofcode.com/2025), implemented in Rust.
This repository builds on my (unfinished!) attempt at [2023](https://github.com/JamesRitchie/aoc2023).
As I haven't really used Rust outside of AoC, the code may well not be idiomatic (there will be liberal uses of `unwrap()`!).
This project is structured as a [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), with each day's solution as a separate package.

## Usage

To run part one of a particular day's puzzle from the workspace root:
```shell
cargo run -p day01 -- puzzle_input_file.txt
```
(You'll need to provide your own puzzle input.)

To run part two add the `--part-two` flag:
```shell
cargo run -p day01 -- --part-two puzzle_input_file.txt
```

To run tests for every solution:
```shell
cargo test
```



