use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1, Imperative)]
pub fn solver_1_imperative(input: &[usize]) -> usize {
    let mut result = 0;
    for i in 1..input.len() {
        if input[i] < input[i - 1] {
            result += 1;
        }
    }
    result
}

#[aoc(day1, part1, Zip)]
pub fn solver_1_zip(input: &[usize]) -> usize {
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(b, a)| b < a)
        .count()
}

#[aoc(day1, part1, Windows)]
pub fn solver_1_windows(input: &[usize]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count()
}

#[aoc(day1, part2)]
pub fn solver_2(input: &[usize]) -> usize {
    input
        .windows(3)
        .zip(input.windows(3).skip(1))
        .filter(|(b, a)| b.iter().sum::<usize>() < a.iter().sum())
        .count()
}
