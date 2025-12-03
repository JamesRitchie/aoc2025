use std::path::PathBuf;

use day03;

#[test]
fn test_part_one() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day03::run(input_path, false).unwrap();
    assert_eq!(answer, 357);
}

#[test]
fn test_part_two() {
    let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("tests/data/sample_input.txt");
    let answer = day03::run(input_path, true).unwrap();
    assert_eq!(answer, 3121910778619);
}
