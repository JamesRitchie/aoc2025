use std::path::PathBuf;

use day08;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day08::run(input_path, 10, false).unwrap();
    assert_eq!(answer, 40);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day08::run(input_path, 10, true).unwrap();
    assert_eq!(answer, 25272);
}
