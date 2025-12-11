use std::{collections::HashMap, error::Error, fs, path::PathBuf};

fn recursive_search<'a>(
    key: &'a str,
    target: &str,
    graph: &HashMap<&str, Vec<&'a str>>,
    lookup: &mut HashMap<&'a str, u64>,
) -> u64 {
    if key == target {
        1
    } else if let Some(cached) = lookup.get(key) {
        *cached
    } else if key == "out" {
        // "Out" is not the target and the DAG ends here.
        0
    } else {
        let values = graph.get(key).expect(&format!("Key {key} not found in graph"));
        let answer = values
            .iter()
            .map(|v| recursive_search(v, target, graph, lookup))
            .sum();
        lookup.insert(key, answer);
        answer
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    let graph = puzzle_input
        .lines()
        .map(|l| {
            let (key, values) = l.split_once(":").expect("No colon in line.");
            let values = values.trim().split_whitespace().collect::<Vec<_>>();
            (key, values)
        })
        .collect::<HashMap<_, _>>();

    if part_two {
        (recursive_search("svr", "dac", &graph, &mut HashMap::new())
            * recursive_search("dac", "fft", &graph, &mut HashMap::new())
            * recursive_search("fft", "out", &graph, &mut HashMap::new()))
            + (recursive_search("svr", "fft", &graph, &mut HashMap::new())
                * recursive_search("fft", "dac", &graph, &mut HashMap::new())
                * recursive_search("dac", "out", &graph, &mut HashMap::new()))
    } else {
        recursive_search("you", "out", &graph, &mut HashMap::new())
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
