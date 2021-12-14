use std::iter;

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

#[aoc(day1, part2, Imperative)]
pub fn solver_2_imperative(input: &[usize]) -> usize {
    let mut sum = input[0] + input[1] + input[2];
    let mut res = 0;

    for start in 0..(input.len() - 3) {
        let previous_sum = sum;
        sum += input[start + 3] - input[start];

        if previous_sum < sum {
            res += 1
        }
    }

    res
}

#[aoc(day1, part2, ZippedWindows)]
pub fn solver_2_zipped_windows(input: &[usize]) -> usize {
    input
        .windows(3)
        .zip(input.windows(3).skip(1))
        .filter(|(b, a)| b.iter().sum::<usize>() < a.iter().sum())
        .count()
}

#[aoc(day1, part2, ScannedWindows)]
pub fn solver_2_scanned_windows(input: &[usize]) -> usize {
    let sums = input.iter().scan(0, |state, &x| {
        *state = *state + x;
        Some(*state)
    });

    let three_sums: Vec<usize> = sums
        .clone()
        .skip(2)
        .zip(iter::once(0usize).chain(sums))
        .map(|(sum, before)| sum - before)
        .collect();

    three_sums.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
mod test {
    use std::iter;

    #[test]
    fn test_scan_sum() {
        let input = vec![1, 2, 3, 4, 5];
        let sums: Vec<usize> = input
            .iter()
            .scan(0, |state, &x| {
                *state = *state + x;
                Some(*state)
            })
            .collect();

        assert_eq!(sums, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_triple_sum() {
        let sums: Vec<usize> = vec![1, 3, 6, 10, 15];

        let three_sums: Vec<usize> = sums
            .iter()
            .skip(2)
            .zip(iter::once(&0).chain(sums.iter()))
            .map(|(sum, before)| sum - before)
            .collect();

        assert_eq!(three_sums, vec![6, 9, 12]);
    }
}
