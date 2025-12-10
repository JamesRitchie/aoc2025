use std::path::PathBuf;

use day10;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day10::run(input_path, false).unwrap();
    assert_eq!(answer, 7);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day10::run(input_path, true).unwrap();
    assert_eq!(answer, 24);
}
