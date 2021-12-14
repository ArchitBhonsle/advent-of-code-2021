use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::BTreeSet, str::FromStr};

use crate::error::Error;

#[derive(Clone, Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Clone, Debug)]
pub struct Fold {
    axis: Axis,
    at: usize,
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if let Some(s) = s.strip_prefix("fold along ") {
            let mut split = s.split("=");
            let axis = match split.next().expect("Could not find the axis") {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => unreachable!(),
            };
            let at = split
                .next()
                .expect("Could not find the at")
                .parse()
                .expect("Could not parse the at");

            Ok(Fold { axis, at })
        } else {
            return Err(Error::new(format!(
                "Line does not start with \"fold along\": {}",
                s
            )));
        }
    }
}

type Coords = (usize, usize);

#[derive(Clone, Debug)]
pub struct Paper(BTreeSet<Coords>);

impl Paper {
    fn fold(&mut self, fold: &Fold) {
        let filter_func = |p: &&Coords| -> bool {
            match fold.axis {
                Axis::X => p.0 > fold.at,
                Axis::Y => p.1 > fold.at,
            }
        };
        let flip_func = |p: &Coords| -> Coords {
            match fold.axis {
                Axis::X => (fold.at - (p.0 - fold.at), p.1),
                Axis::Y => (p.0, fold.at - (p.1 - fold.at)),
            }
        };

        let flipped_points: Vec<Coords> =
            self.0.iter().filter(filter_func).map(flip_func).collect();

        self.0.retain(|p| !filter_func(&&p));
        self.0.extend(flipped_points.iter());
    }
}

impl FromStr for Paper {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: BTreeSet<Coords> = s
            .trim()
            .lines()
            .map(|l| {
                let mut split = l.split(",");
                let x = split
                    .next()
                    .expect("Could not find x")
                    .parse()
                    .expect("Could not parse x");
                let y = split
                    .next()
                    .expect("Could not find y")
                    .parse()
                    .expect("Could not parse y");

                (x, y)
            })
            .collect();

        Ok(Paper(points))
    }
}

type Input = (Paper, Vec<Fold>);

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let paper = split
        .next()
        .expect("Could not find paper")
        .parse()
        .expect("Could not parse paper");
    let folds = split
        .next()
        .expect("Could not find folds")
        .split("\n")
        .map(|f| f.parse().expect("Could not parse fold"))
        .collect();

    (paper, folds)
}

#[aoc(day13, part1)]
pub fn solver_1(input: &Input) -> usize {
    let (mut paper, folds) = input.clone();

    paper.fold(folds.first().expect("Could not find first fold"));

    paper.0.len()
}

#[aoc(day13, part2)]
pub fn solver_2(input: &Input) -> usize {
    let (mut paper, folds) = input.clone();

    folds.iter().for_each(|f| paper.fold(f));

    let (x_max, y_max) = (
        paper
            .0
            .iter()
            .max_by_key(|p| p.0)
            .expect("Could not find maximum x")
            .0
            + 1,
        paper
            .0
            .iter()
            .max_by_key(|p| p.1)
            .expect("Could not find maximum y")
            .1
            + 1,
    );

    for y in 0..y_max {
        for x in 0..x_max {
            print!("{}", if paper.0.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }

    paper.0.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn example_1() {
        let input = generator(INPUT);
        let result = solver_1(&input);

        assert_eq!(result, 17);
    }

    #[test]
    fn example_2() {
        let input = generator(INPUT);
        let result = solver_2(&input);

        assert_eq!(result, 16);
    }
}
