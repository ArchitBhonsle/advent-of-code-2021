use aoc_runner_derive::{aoc, aoc_generator};

fn fuel_needed_constant(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}

fn fuel_needed_linear(x: usize, y: usize) -> usize {
    let diff = if x > y { x - y } else { y - x };

    (diff * (diff + 1)) / 2
}

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|crab| crab.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solver_1(crabs: &Vec<usize>) -> usize {
    let min = *crabs.iter().min().unwrap() as usize;
    let max = *crabs.iter().max().unwrap() as usize;

    (min..=max)
        .map(|align| {
            crabs
                .iter()
                .map(|crab| fuel_needed_constant(align, *crab))
                .sum()
        })
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn solver_2(crabs: &Vec<usize>) -> usize {
    let min = *crabs.iter().min().unwrap() as usize;
    let max = *crabs.iter().max().unwrap() as usize;

    (min..=max)
        .map(|align| {
            crabs
                .iter()
                .map(|crab| fuel_needed_linear(align, *crab))
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_example_1() {
        let crabs = generator(INPUT);
        let result = solver_1(&crabs);

        assert_eq!(result, 37);
    }

    #[test]
    fn test_example_2() {
        let crabs = generator(INPUT);
        let result = solver_2(&crabs);

        assert_eq!(result, 168);
    }
}
