use std::path::PathBuf;

use day11;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_1.txt");
    let answer = day11::run(input_path, false).unwrap();
    assert_eq!(answer, 5);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input_2.txt");
    let answer = day11::run(input_path, true).unwrap();
    assert_eq!(answer, 2);
}
