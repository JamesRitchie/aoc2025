use std::{cmp::max, error::Error, fs, ops::RangeInclusive, path::PathBuf};

fn merge_ranges<T>(ranges: &Vec<RangeInclusive<T>>) -> Vec<RangeInclusive<T>>
where
    T: Ord + Copy,
{
    // Merge overlapping ranges and return a sorted list of non-overlapping ranges

    if ranges.is_empty() {
        return Vec::new();
    }

    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| *r.start());

    let mut merged_ranges = Vec::new();
    let mut current_range = sorted_ranges[0].clone();

    for r in &sorted_ranges[1..] {
        if *current_range.end() >= *r.start() {
            current_range = *current_range.start()..=max(*current_range.end(), *r.end());
        } else {
            merged_ranges.push(current_range);
            current_range = r.clone();
        }
    }

    merged_ranges.push(current_range);
    merged_ranges
}

fn check_ranges<T>(ranges: &Vec<RangeInclusive<T>>, value: T) -> bool
where
    T: Ord,
{
    // Do a binary search since the ranges are non-overlapping and sorted
    // Not strictly necessary for the given input, but as we've sorted them anyway...
    if ranges.is_empty() {
        return false;
    } else if value < *ranges.first().unwrap().start() || value > *ranges.last().unwrap().end() {
        false
    } else {
        let mut low = 0;
        let mut high = ranges.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let r = &ranges[mid];
            if r.contains(&value) {
                return true;
            } else if value < *r.start() {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        false
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    let (ranges_input, ids_input) = puzzle_input
        .split_once("\n\n")
        .expect("Puzzle input format wrong");

    let unmerged_ranges = ranges_input
        .lines()
        .map(|l| {
            let (lower, upper) = l
                .split_once("-")
                .expect("Range input should be delimited by -");
            let lower: u64 = lower.parse().expect("Range start should be an integer");
            let upper: u64 = upper.parse().expect("Range end should be an integer");
            lower..=upper
        })
        .collect::<Vec<_>>();

    let merged_ranges = merge_ranges(&unmerged_ranges);

    if part_two {
        merged_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    } else {
        ids_input
            .lines()
            .map(|l| l.parse::<u64>().expect("ID should be an integer"))
            .filter(|id| check_ranges(&merged_ranges, *id))
            .count() as u64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
