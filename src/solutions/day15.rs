use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use dynamic_matrix::DynamicMatrix;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> DynamicMatrix<usize> {
    let mut grid = DynamicMatrix::new_with_cols(input.lines().last().unwrap().len());

    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as usize)
                .collect()
        })
        .for_each(|v| {
            grid.push_row(v).unwrap();
        });

    grid
}

#[aoc(day15, part1)]
pub fn solver_1(grid: &DynamicMatrix<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn example_1() {
        let grid = generator(INPUT);
        let result = solver_1(&grid);

        assert_eq!(result, 40);
    }
}
