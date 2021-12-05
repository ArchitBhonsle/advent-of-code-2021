use std::str::FromStr;
use std::{cmp, error::Error};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn minmax<T: Ord + Clone>(x: &T, y: &T) -> (T, T) {
    (
        cmp::min(x.clone(), y.clone()),
        cmp::max(x.clone(), y.clone()),
    )
}

#[derive(Debug, Clone)]
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

    fn get_intersection_length(&self, other: &Self) -> isize {
        // check if intersecting but parallel

        // if they are vertical
        if self.is_vertical() && other.is_vertical() && self.start.x == other.start.x {
            let (self_min, self_max) = minmax(&self.start.x, &self.start.x);
            let (other_min, other_max) = minmax(&other.start.x, &other.start.x);

            return cmp::min(self_max, other_max) - cmp::max(self_min, other_min);
        }

        // if they are horizontal
        if self.is_horizontal() && other.is_horizontal() && self.start.y == other.start.y {
            let (self_min, self_max) = minmax(&self.start.y, &self.start.y);
            let (other_min, other_max) = minmax(&other.start.y, &other.start.y);

            return cmp::min(self_max, other_max) - cmp::max(self_min, other_min);
        }

        // if intersecting but not parallel
        let (self_min, self_max) = minmax(&self.start.x, &self.end.x);
        if self_min <= other.start.y && other.start.y <= self_max {
            return 1;
        }

        0
    }

    fn contains(&self, p: &Point) -> bool {
        let x_inc = match Ord::cmp(&self.start.x, &self.end.x) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };
        let y_inc = match Ord::cmp(&self.start.y, &self.end.y) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };
        let (mut x, mut y) = (self.start.x, self.start.y);
        let mut res = false;

        // println!("{:?} {} {} => {} {}", self, x_inc, y_inc, p.x, p.y);
        loop {
            // println!("{} {} {} {}", x, y, p.x, p.y);
            if x == p.x && y == p.y {
                res = true;
                break;
            }

            x += x_inc;
            y += y_inc;

            if self.end.x == x && self.end.y == y {
                if x == p.x && y == p.y {
                    res = true;
                }
                break;
            }
        }
        res
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

#[aoc(day5, part1)]
pub fn solver_1(lines: &Vec<Line>) -> isize {
    lines
        .iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .tuple_combinations()
        .map(|(line1, line2)| Line::get_intersection_length(&line1, &line2))
        .sum()
}

#[aoc(day5, part1, ugly)]
pub fn solver_1_ugly(lines: &Vec<Line>) -> isize {
    let lines = lines
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .map(|line| line.clone())
        .collect::<Vec<Line>>();

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
    for x in 0..max_x {
        for y in 0..max_y {
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

#[aoc(day5, part2, ugly)]
pub fn solver_2_ugly(lines: &Vec<Line>) -> isize {
    let max_x = lines
        .iter()
        .map(|line| cmp::max(line.start.x, line.end.x))
        .max()
        .unwrap()
        + 1;

    let max_y = lines
        .iter()
        .map(|line| cmp::max(line.start.y, line.end.y))
        .max()
        .unwrap()
        + 1;
    // dbg!(max_x, max_y);

    // let mut matrix = vec![vec![0; max_x as usize]; max_y as usize];

    let mut res = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            let mut count = 0;
            for line in lines.iter() {
                if line.contains(&Point { x, y }) {
                    // matrix[y as usize][x as usize] += 1;
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
    // println!(
    //     "{}",
    //     matrix
    //         .into_iter()
    //         .map(|row| {
    //             row.into_iter()
    //                 .map(|num| num.to_string())
    //                 .collect::<Vec<String>>()
    //                 .join(" ")
    //         })
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    res
}

#[cfg(test)]
mod test {
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
    //     fn example_1() {
    //         let lines = generator(INPUT).unwrap();
    //         let result = solver_1(&lines);

    //         assert_eq!(result, 5);
    //     }

    //     #[test]
    //     fn example_1_ugly() {
    //         let lines = generator(INPUT).unwrap();
    //         let result = solver_1_ugly(&lines);

    //         assert_eq!(result, 5);
    //     }

    // #[test]
    // fn test_contains() {
    //     assert!(Line::from_str("0,0 -> 10,10")
    //         .unwrap()
    //         .contains(&Point::from_str("5,5").unwrap()));
    //     assert!(Line::from_str("10,10 -> 0,0")
    //         .unwrap()
    //         .contains(&Point::from_str("5,5").unwrap()));
    //     assert!(Line::from_str("0,10 -> 10,0")
    //         .unwrap()
    //         .contains(&Point::from_str("5,5").unwrap()));
    //     assert!(Line::from_str("10,0 -> 0,10")
    //         .unwrap()
    //         .contains(&Point::from_str("5,5").unwrap()));
    // }
    #[test]
    fn example_2_ugly() {
        println!("started");
        let lines = generator(INPUT).unwrap();
        let result = solver_2_ugly(&lines);

        assert_eq!(result, 12);
    }
}
