use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

type Movements = Vec<Movement>;

pub enum Movement {
    Forward(isize),
    Up(isize),
    Down(isize),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Result<Movements, Box<dyn Error>> {
    input
        .lines()
        .map(|m| {
            let mut contents = m.split(' ');
            let direction = contents.next().ok_or("Direction not found")?;
            let magnitude = contents.next().ok_or("Magnitude not found")?.parse()?;

            Ok(match direction {
                "forward" => Movement::Forward(magnitude),
                "up" => Movement::Up(magnitude),
                "down" => Movement::Down(magnitude),
                _ => unreachable!(),
            })
        })
        .collect::<Result<Movements, Box<dyn Error>>>()
}

#[aoc(day2, part1)]
pub fn solver_1(input: &Movements) -> isize {
    let (mut horizontal, mut vertical) = (0, 0);

    for m in input {
        match m {
            &Movement::Up(x) => {
                vertical -= x;
            }
            &Movement::Down(x) => {
                vertical += x;
            }
            &Movement::Forward(x) => {
                horizontal += x;
            }
        }
    }

    horizontal * vertical
}

#[aoc(day2, part1, Filters)]
pub fn solver_1_filters(input: &Movements) -> isize {
    let horizontal: isize = input
        .iter()
        .filter(|x| match x {
            &Movement::Forward(_) => true,
            _ => false,
        })
        .map(|x| match x {
            &Movement::Forward(x) => x,
            _ => unreachable!(),
        })
        .sum();

    let vertical: isize = input
        .iter()
        .filter(|x| match x {
            &Movement::Forward(_) => false,
            _ => true,
        })
        .map(|x| match x {
            &Movement::Up(x) => -x,
            &Movement::Down(x) => x,
            _ => unreachable!(),
        })
        .sum();

    horizontal * vertical
}

#[aoc(day2, part2)]
pub fn solver_2(input: &Movements) -> isize {
    let (mut aim, mut horizontal, mut vertical) = (0, 0, 0);

    for m in input {
        match m {
            &Movement::Up(x) => {
                aim -= x;
            }
            &Movement::Down(x) => {
                aim += x;
            }
            &Movement::Forward(x) => {
                horizontal += x;
                vertical += aim * x;
            }
        }
    }

    horizontal * vertical
}
