use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use itertools::Itertools;

struct UnionFind {
    capacity: usize,
    parents: HashMap<usize, usize>,
}

impl UnionFind {
    fn new(capacity: usize) -> UnionFind {
        UnionFind {
            capacity: capacity,
            parents: (0..capacity).map(|i| (i, i)).collect(),
        }
    }

    fn find(&mut self, i: usize) -> Option<usize> {
        if let Some(parent) = self.parents.get(&i) {
            if i == *parent {
                Some(i)
            } else {
                let root = self.find(*parent).expect("");
                self.parents.insert(i, root);
                Some(root)
            }
        } else {
            None
        }
    }

    fn union(&mut self, i: usize, j: usize) -> Result<(), &str> {
        let Some(parent_i) = self.find(i) else {
            return Err("i not in sets");
        };
        let Some(parent_j) = self.find(j) else {
            return Err("j not in sets");
        };
        if parent_i != parent_j {
            self.parents.insert(parent_i, parent_j);
            return Ok(());
        } else {
            return Ok(());
        }
    }

    fn set_sizes(&mut self) -> HashMap<usize, usize> {
        (0..self.capacity)
            .map(|i| self.find(i).expect("Item not in sets"))
            .fold(HashMap::new(), |mut acc, p| {
                *acc.entry(p).or_insert(0) += 1;
                acc
            })
    }
}

fn squared_distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    let dz = (a.2 - b.2) as i64;
    dx.pow(2) + dy.pow(2) + dz.pow(2)
}

fn compute_answer(puzzle_input: &str, pairs: usize, part_two: bool) -> u64 {
    let junction_boxes = puzzle_input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();
            (coords[0], coords[1], coords[2])
        })
        .collect::<Vec<(i32, i32, i32)>>();

    let mut pairwise_distances = (0..junction_boxes.len())
        .combinations(2)
        .map(|c| {
            (
                c[0],
                c[1],
                squared_distance(junction_boxes[c[0]], junction_boxes[c[1]]),
            )
        })
        .collect::<Vec<_>>();

    pairwise_distances.sort_by_key(|p| p.2);

    let mut union_find = UnionFind::new(junction_boxes.len());

    let mut connections = 0;
    for i in 0..pairwise_distances.len() {
        let (p1, p2, _) = pairwise_distances[i];

        if i == pairs && !part_two {
            return union_find
                .set_sizes()
                .values()
                .sorted()
                .rev()
                .take(3)
                .map(|s| *s as u64)
                .product();
        }

        if union_find.find(p1) != union_find.find(p2) {
            union_find
                .union(p1, p2)
                .expect("Both keys should be in sets");
            connections += 1;
            if connections == junction_boxes.len() - 1 {
                return (junction_boxes[p1].0 * junction_boxes[p2].0) as u64;
            }
        }
    }

    panic!("Failed to connect all junction boxes")
}

pub fn run(input_path: PathBuf, pairs: usize, part_two: bool) -> Result<u64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, pairs, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
