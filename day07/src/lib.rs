use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs,
    path::PathBuf,
};

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
struct GridPoint {
    x: usize,
    y: usize,
}

fn recursive_search(
    point: GridPoint,
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<GridPoint, u64>,
    splitters: &mut HashSet<GridPoint>,
) -> u64 {
    // Recursively search the grid for all possible paths from the current point to the bottom
    // whilst logging visited splitters, using memoization to cache results.
    if let Some(result) = cache.get(&point) {
        return *result;
    }

    let n = grid.len();
    let m = grid[0].len();
    let paths;
    if point.y == n {
        paths = 1;
    } else if point.x == m {
        paths = 0;
    } else {
        let cell = grid[point.y][point.x];

        paths = match cell {
            'S' | '.' => recursive_search(
                GridPoint {
                    x: point.x,
                    y: point.y + 1,
                },
                grid,
                cache,
                splitters,
            ),
            '^' => {
                splitters.insert(point);
                recursive_search(
                    GridPoint {
                        x: point.x.saturating_sub(1),
                        y: point.y,
                    },
                    grid,
                    cache,
                    splitters,
                ) + recursive_search(
                    GridPoint {
                        x: point.x + 1,
                        y: point.y,
                    },
                    grid,
                    cache,
                    splitters,
                )
            }
            _ => panic!("Invalid cell char"),
        }
    }
    cache.insert(point, paths);
    paths
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> u64 {
    let grid = puzzle_input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = GridPoint {
        x: grid[0]
            .iter()
            .position(|c| *c == 'S')
            .expect("Start should be on first line"),
        y: 0,
    };

    let mut recursion_cache: HashMap<GridPoint, u64> = HashMap::new();
    let mut visited_splitters: HashSet<GridPoint> = HashSet::new();

    let path = recursive_search(start, &grid, &mut recursion_cache, &mut visited_splitters);

    if part_two {
        path
    } else {
        visited_splitters.len() as u64
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
