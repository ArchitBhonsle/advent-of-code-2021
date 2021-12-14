use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = (Vec<char>, HashMap<(char, char), char>);

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Input {
    let mut input_split = input.split("\n\n");

    let polymer = input_split
        .next()
        .expect("Initial polymer not found")
        .trim()
        .chars()
        .collect();

    let rules = input_split
        .next()
        .expect("Insertion rules not found")
        .lines()
        .map(|line| {
            let mut line_split = line.trim().split("->").map(|part| part.trim());

            let pair: (char, char) = line_split
                .next()
                .expect("Insertion pair not found")
                .chars()
                .collect_tuple()
                .expect("Could not create the pair tuple");

            let insert = line_split
                .next()
                .expect("Result of insertion not found")
                .chars()
                .next()
                .expect("Result of insertion not found");

            (pair, insert)
        })
        .collect();

    (polymer, rules)
}

#[aoc(day14, part1)]
pub fn solver_1(input: &Input) -> usize {
    let (mut prev, map) = input.clone();

    let mut next: Vec<char> = Vec::new();
    for _ in 0..10 {
        for (a, b) in prev.iter().tuple_windows() {
            next.push(*a);
            if let Some(insert) = map.get(&(*a, *b)) {
                next.push(*insert);
            }
        }

        next.push(*prev.last().expect("Last element not found"));
        prev = next;
        next = Vec::new();
    }

    let count_map: HashMap<char, usize> = prev.iter().fold(HashMap::new(), |mut count, c| {
        *count.entry(*c).or_insert(0) += 1;
        count
    });

    let min = count_map
        .iter()
        .min_by_key(|c| c.1)
        .expect("Could not find minimum")
        .1;
    let max = count_map
        .iter()
        .max_by_key(|c| c.1)
        .expect("Could not find minimum")
        .1;

    max - min
}

pub struct PolymerizationCache {
    cache: HashMap<(char, char, usize), HashMap<char, usize>>,
    map: HashMap<(char, char), char>,
}

impl PolymerizationCache {
    pub fn new(map: HashMap<(char, char), char>) -> Self {
        PolymerizationCache {
            cache: HashMap::new(),
            map,
        }
    }

    pub fn lookup(&mut self, pair: (char, char), depth: usize) -> HashMap<char, usize> {
        if depth > 40 {
            HashMap::new()
        } else {
            let result = if let Some(result) = self.cache.get(&(pair.0, pair.1, depth)) {
                result.clone()
            } else if let Some(insert) = self.map.get(&pair) {
                let insert = *insert;
                let mut left = self.lookup((pair.0, insert), depth + 1);
                let right = self.lookup((insert, pair.1), depth + 1);

                right
                    .into_iter()
                    .for_each(|(k, v)| *left.entry(k).or_insert(0) += v);

                *left.entry(insert).or_insert(0) += 1;

                left
            } else {
                HashMap::new()
            };

            self.cache.insert((pair.0, pair.1, depth), result.clone());

            result
        }
    }
}

#[aoc(day14, part2)]
pub fn solver_2(input: &Input) -> usize {
    let mut count = HashMap::new();
    let mut cache = PolymerizationCache::new(input.1.clone());

    for (a, b) in input.0.iter().tuple_windows() {
        let result = cache.lookup((*a, *b), 1);

        result
            .into_iter()
            .for_each(|(k, v)| *count.entry(k).or_insert(0) += v);

        *count.entry(*a).or_insert(0) += 1;
    }
    *count
        .entry(*input.0.last().expect("Last character could not be found"))
        .or_insert(0) += 1;

    let min = count
        .iter()
        .min_by_key(|c| c.1)
        .expect("Could not find minimum")
        .1;
    let max = count
        .iter()
        .max_by_key(|c| c.1)
        .expect("Could not find minimum")
        .1;

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn example_1() {
        let input = generator(INPUT);
        let result = solver_1(&input);

        assert_eq!(result, 1588);
    }

    #[test]
    fn example_2() {
        let input = generator(INPUT);
        let result = solver_2(&input);

        assert_eq!(result, 2188189693529);
    }
}
