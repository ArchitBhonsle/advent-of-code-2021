use std::str::FromStr;
use std::{cmp, error::Error};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim().splitn(2, ',').collect();
        let x = coords.get(0).ok_or("x could not be found")?.parse()?;
        let y = coords.get(1).ok_or("y could not be found")?.parse()?;

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        if self.start.x == self.end.x {
            true
        } else {
            false
        }
    }

    fn is_horizontal(&self) -> bool {
        if self.start.y == self.end.y {
            true
        } else {
            false
        }
    }

    fn slope(&self) -> (isize, isize) {
        (
            match cmp::Ord::cmp(&self.start.x, &self.end.x) {
                cmp::Ordering::Less => -1,
                cmp::Ordering::Equal => 0,
                cmp::Ordering::Greater => 1,
            },
            match cmp::Ord::cmp(&self.start.y, &self.end.y) {
                cmp::Ordering::Less => -1,
                cmp::Ordering::Equal => 0,
                cmp::Ordering::Greater => 1,
            },
        )
    }

    fn contains(&self, p: &Point) -> bool {
        // points at the beginning and the end of the line will give weird slopes
        // so we take care of this first
        if &self.start == p || p == &self.end {
            return true;
        }

        let to_p = Line {
            start: self.start.clone(),
            end: p.clone(),
        };

        let slope = self.slope();
        let to_p_slope = to_p.slope();
        if slope != to_p_slope {
            return false;
        }

        // these combined with the above slope check confirm that the point `p` is inside the rectangle
        // created by `self.start` and `self.end`
        let x_bounds = (p.x - self.start.x).abs() < (self.end.x - self.start.x).abs();
        let y_bounds = (p.y - self.start.y).abs() < (self.end.y - self.start.y).abs();

        if slope.0 == 0 {
            y_bounds
        } else if slope.1 == 0 {
            x_bounds
        } else {
            x_bounds
                && y_bounds
                && ((p.x - self.start.x) / slope.0 == (p.y - self.start.y) / slope.1)
            // checks if the point is on the line
        }
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.trim().splitn(2, "->").collect();
        let start = points.get(0).ok_or("start could not be found")?.parse()?;
        let end = points.get(1).ok_or("end could not be found")?.parse()?;

        Ok(Self { start, end })
    }
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Result<Vec<Line>, Box<dyn Error>> {
    input.lines().map(|line| Ok(line.trim().parse()?)).collect()
}

fn count_intersections_imperative(lines: &Vec<Line>) -> usize {
    let max_x = lines
        .iter()
        .map(|line| cmp::max(line.start.x, line.end.x))
        .max()
        .unwrap();
    let max_y = lines
        .iter()
        .map(|line| cmp::max(line.start.y, line.end.y))
        .max()
        .unwrap();

    let mut res = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let mut count = 0;
            for line in lines.iter() {
                if line.contains(&Point { x, y }) {
                    count += 1
                }
                if count > 1 {
                    break;
                }
            }
            if count > 1 {
                res += 1;
            }
        }
    }

    res
}

fn count_intersections_functional(lines: &Vec<Line>) -> usize {
    let max_x = lines
        .iter()
        .map(|line| cmp::max(line.start.x, line.end.x))
        .max()
        .unwrap() as usize;
    let max_y = lines
        .iter()
        .map(|line| cmp::max(line.start.y, line.end.y))
        .max()
        .unwrap() as usize;

    (0..=max_x)
        .map(|x| (0..=max_y).map(move |y| (x as isize, y as isize)))
        .flatten()
        .filter(|(x, y)| {
            lines
                .iter()
                .filter(|line| line.contains(&Point { x: *x, y: *y }))
                .count()
                > 1
        })
        .count()
}

#[aoc(day5, part1, Imperative)]
pub fn solver_1_imperative(lines: &Vec<Line>) -> usize {
    let lines = lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .map(|line| line.clone())
        .collect::<Vec<Line>>();

    count_intersections_imperative(&lines)
}

#[aoc(day5, part1, Functional)]
pub fn solver_1_functional(lines: &Vec<Line>) -> usize {
    let lines = lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .map(|line| line.clone())
        .collect::<Vec<Line>>();

    count_intersections_functional(&lines)
}

#[aoc(day5, part2, Imperative)]
pub fn solver_2_imperative(lines: &Vec<Line>) -> usize {
    count_intersections_imperative(lines)
}

#[aoc(day5, part2, Functional)]
pub fn solver_2_functional(lines: &Vec<Line>) -> usize {
    count_intersections_functional(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_contains() {
        let check = |line, point| {
            assert!(Line::from_str(line)
                .unwrap()
                .contains(&Point::from_str(point).unwrap()));
        };

        check("0,0 -> 10,10", "5,5");
        check("10,10 -> 0,0", "5,5");
        check("0,10 -> 10,0", "5,5");
        check("10,0 -> 0,10", "5,5");
    }

    #[test]
    fn test_example_1() {
        let lines = generator(INPUT).unwrap();
        let result_imperative = solver_1_imperative(&lines);
        let result_functional = solver_1_functional(&lines);

        assert_eq!(result_imperative, 5);
        assert_eq!(result_functional, 5);
    }

    #[test]
    fn test_example_2() {
        let lines = generator(INPUT).unwrap();
        let result_imperative = solver_2_imperative(&lines);
        let result_functional = solver_2_functional(&lines);

        assert_eq!(result_imperative, 12);
        assert_eq!(result_functional, 12);
    }
}
