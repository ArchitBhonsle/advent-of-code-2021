use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Caves = HashMap<String, Vec<String>>;

fn is_start(x: &String) -> bool {
    x == "start"
}
fn is_end(x: &String) -> bool {
    x == "end"
}
fn is_big(x: &String) -> bool {
    !is_start(x) && !is_end(x) && x.chars().all(|c| c.is_uppercase())
}
fn is_small(x: &String) -> bool {
    !is_start(x) && !is_end(x) && x.chars().all(|c| c.is_lowercase())
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Caves {
    let mut caves: Caves = HashMap::new();

    let paths: Vec<(String, String)> = input
        .trim()
        .lines()
        .map(|line| {
            let caves: Vec<String> = line.split('-').map(|c| c.to_string()).collect();

            (caves[0].clone(), caves[1].clone())
        })
        .collect();

    for (a, b) in paths {
        caves.entry(a.clone()).or_default().push(b.clone());
        caves.entry(b.clone()).or_default().push(a.clone());
    }

    caves
}

fn traversal_1(curr: &String, caves: &Caves, mut visited: HashSet<String>) -> usize {
    if is_end(curr) {
        return 1;
    }

    visited.insert(curr.clone());
    caves[curr]
        .iter()
        .filter(|x| is_big(x) || !visited.contains(*x))
        .map(|x| traversal_1(x, caves, visited.clone()))
        .sum()
}

#[aoc(day12, part1)]
pub fn solver_1(caves: &Caves) -> usize {
    traversal_1(&"start".to_string(), caves, HashSet::new())
}

fn traversal_2(
    curr: &String,
    caves: &Caves,
    mut visited: HashSet<String>,
    twice_used: bool,
) -> usize {
    if is_end(curr) {
        return 1;
    }

    visited.insert(curr.clone());
    if twice_used {
        caves[curr]
            .iter()
            .filter(|x| is_big(x) || !visited.contains(*x))
            .map(|x| traversal_2(x, caves, visited.clone(), true))
            .sum()
    } else {
        caves[curr]
            .iter()
            .filter(|x| !is_start(x))
            .map(|x| {
                if is_small(x) && visited.contains(x) {
                    traversal_2(x, caves, visited.clone(), true)
                } else {
                    traversal_2(x, caves, visited.clone(), false)
                }
            })
            .sum()
    }
}

#[aoc(day12, part2)]
pub fn solver_2(caves: &Caves) -> usize {
    traversal_2(&"start".to_string(), caves, HashSet::new(), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    fn check_1(input: &str, value: usize) {
        let caves = generator(input);
        let result = solver_1(&caves);
        assert_eq!(result, value);
    }

    #[test]
    fn example_1() {
        check_1(INPUT_1, 10);
        check_1(INPUT_2, 19);
        check_1(INPUT_3, 226);
    }

    fn check_2(input: &str, value: usize) {
        let caves = generator(input);
        let result = solver_2(&caves);
        assert_eq!(result, value);
    }

    #[test]
    fn example_2() {
        check_2(INPUT_1, 36);
        check_2(INPUT_2, 103);
        check_2(INPUT_3, 3509);
    }
}
