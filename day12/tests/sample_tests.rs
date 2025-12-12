use std::path::PathBuf;

use day12;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day12::run(input_path, false).unwrap();
    // This is the wrong answer, as the real input does not require us to solve the packing problem.
    assert_eq!(answer, 1);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day12::run(input_path, true).unwrap();
    // No part two today.
    assert_eq!(answer, 0);
}
