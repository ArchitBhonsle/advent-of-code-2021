use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

use aoc_runner_derive::{aoc, aoc_generator};
use dynamic_matrix::DynamicMatrix;

type Grid = DynamicMatrix<usize>;
type Coords = (usize, usize);

fn valid_neighbours(
    grid: &DynamicMatrix<usize>,
    coords: Coords,
    part2: bool,
) -> Vec<(usize, usize)> {
    let lim = if !part2 { 1 } else { 5 };
    let (row, col) = (coords.0 as isize, coords.1 as isize);
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|(rd, cd)| (row + rd, col + cd))
        .filter(|(r, c)| {
            0 <= *r
                && *r < (grid.rows() * lim) as isize
                && 0 <= *c
                && *c < (grid.cols() * lim) as isize
        })
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Grid {
    let mut grid = Grid::new_with_cols(input.lines().last().unwrap().len());

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

#[derive(Eq, PartialEq)]
struct HeapItem(usize, Coords);

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day15, part1)]
pub fn solver_1(grid: &Grid) -> usize {
    let (start, end) = ((0, 0), (grid.rows() - 1, grid.cols() - 1));

    let mut dist: HashMap<Coords, usize> = HashMap::new();
    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(HeapItem(0, start));

    while let Some(HeapItem(cost, coord)) = heap.pop() {
        if coord == end {
            // for r in 0..grid.rows() {
            //     for c in 0..grid.cols() {
            //         let x = dist.get(&(r, c)).or_else(|| Some(&0)).unwrap();
            //         print!("{:^3}", x);
            //     }
            //     println!();
            // }
            return cost;
        }
        if cost > *dist.entry(coord).or_insert(usize::MAX) {
            continue;
        }

        for n in valid_neighbours(grid, coord, false) {
            let next = HeapItem(cost + grid[n], n);

            if next.0 < *dist.entry(n).or_insert(usize::MAX) {
                dist.insert(n, next.0);
                heap.push(next);
            }
        }
    }

    usize::MAX
}

fn part2_index(grid: &Grid, index: (usize, usize)) -> usize {
    let mod_by = grid.rows();
    let (row, col) = index;
    let (original_row, original_col) = (row % mod_by, col % mod_by);
    let (delta_row, delta_col) = (row / mod_by, col / mod_by);

    ((grid[(original_row, original_col)] - 1 + delta_row + delta_col) % 9) + 1
}

#[aoc(day15, part2)]
pub fn solver_2(grid: &Grid) -> usize {
    let (start, end) = ((0, 0), (grid.rows() * 5 - 1, grid.cols() * 5 - 1));

    let mut dist: HashMap<Coords, usize> = HashMap::new();
    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(HeapItem(0, start));

    while let Some(HeapItem(cost, coord)) = heap.pop() {
        if coord == end {
            return cost;
        }
        if cost > *dist.entry(coord).or_insert(usize::MAX) {
            continue;
        }

        for n in valid_neighbours(grid, coord, true) {
            let next = HeapItem(cost + part2_index(grid, n), n);

            if next.0 < *dist.entry(n).or_insert(usize::MAX) {
                dist.insert(n, next.0);
                heap.push(next);
            }
        }
    }

    usize::MAX
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

    #[test]
    fn example_2() {
        let grid = generator(INPUT);
        let result = solver_2(&grid);

        assert_eq!(result, 315);
    }

    #[test]
    fn grid_check() {
        let grid = generator(INPUT);

        for r in 0..(grid.rows() * 5) {
            for c in 0..(grid.cols() * 5) {
                let x = part2_index(&grid, (r, c));
                print!("{:^2}", x);
            }
            println!();
        }
    }
}
