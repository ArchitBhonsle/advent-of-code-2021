use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

pub struct Cave {
    data: Vec<usize>,
    rows: usize,
    cols: usize,
}

type CaveIndex = (isize, isize);

impl Cave {
    fn index(&self, index: CaveIndex) -> Option<usize> {
        let (rows, cols) = index;
        if rows < 0 || cols < 0 {
            None
        } else if rows as usize >= self.rows || cols as usize >= self.cols {
            None
        } else {
            Some(self.data[self.cols * rows as usize + cols as usize])
        }
    }

    fn get_neighbourhood_min(&self, index: CaveIndex) -> CaveIndex {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|(rd, cd)| {
                let index = (index.0 + rd, index.1 + cd);
                let value = self.index(index);

                (value, index)
            })
            .filter(|(value, _)| value.is_some())
            .map(|(value, index)| (value.unwrap(), index))
            .min_by_key(|(value, _)| *value)
            .unwrap()
            .1
    }
}

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Cave {
    let mut data: Vec<usize> = Vec::new();
    let (mut rows, mut cols) = (0, 0);
    for line in input.split('\n') {
        let line_vec: Vec<usize> = line
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect();

        cols = line_vec.len();
        rows += 1;

        data.extend(line_vec);
    }

    Cave { data, rows, cols }
}

#[aoc(day9, part1)]
pub fn solver_1(cave: &Cave) -> usize {
    (0..cave.rows)
        .into_iter()
        .flat_map(|row| (0..cave.cols).into_iter().map(move |col| (row, col)))
        .map(|(row, col)| (row as isize, col as isize))
        .filter(|index| {
            let nm_index = cave.get_neighbourhood_min(*index);
            cave.index(*index).unwrap() < cave.index(nm_index).unwrap()
        })
        .map(|index| cave.index(index).unwrap() + 1)
        .sum()
}

// stack recursion dfs
fn get_set_basin(
    basin_map: &mut HashMap<CaveIndex, CaveIndex>,
    cave: &Cave,
    index: CaveIndex,
) -> CaveIndex {
    let value = cave.index(index);
    let nm_index = cave.get_neighbourhood_min(index);
    let nm_value = cave.index(nm_index);

    if value < nm_value {
        basin_map.insert(index, index);
        return index;
    }

    let basin = get_set_basin(basin_map, cave, nm_index);
    basin_map.insert(index, basin);

    return basin;
}

#[aoc(day9, part2)]
pub fn solver_2(cave: &Cave) -> usize {
    let mut basin_map: HashMap<CaveIndex, CaveIndex> = HashMap::new();

    (0..cave.rows)
        .into_iter()
        .flat_map(|row| (0..cave.cols).into_iter().map(move |col| (row, col)))
        .map(|(row, col)| (row as isize, col as isize))
        .filter(|index| cave.index(*index).unwrap() != 9)
        .for_each(|index| {
            get_set_basin(&mut basin_map, cave, index);
        });

    let mut basin_size_map: HashMap<CaveIndex, usize> = HashMap::new();
    basin_map.values().for_each(|basin| {
        *basin_size_map.entry(*basin).or_default() += 1;
    });

    basin_size_map
        .values()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn example_1() {
        let cave = generator(INPUT);
        let result = solver_1(&cave);

        assert_eq!(result, 15);
    }

    #[test]
    fn example_2() {
        let cave = generator(INPUT);
        let result = solver_2(&cave);

        assert_eq!(result, 1134);
    }
}
