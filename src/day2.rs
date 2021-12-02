use aoc_runner_derive::{aoc, aoc_generator};

type Movements = Vec<Movement>;

pub enum Movement {
    Forward(isize),
    Up(isize),
    Down(isize),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Movements {
    input
        .lines()
        .map(|x| {
            let mut contents = x.split(' ');
            let direction = contents.next().unwrap();
            let magnitude = contents.next().unwrap().parse().unwrap();

            match direction {
                "forward" => Movement::Forward(magnitude),
                "up" => Movement::Up(magnitude),
                "down" => Movement::Down(magnitude),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solver_1(input: &Movements) -> isize {
    let horizontal = input
        .iter()
        .filter(|x| match x {
            &Movement::Forward(_) => true,
            _ => false,
        })
        .map(|x| match x {
            &Movement::Forward(x) => x,
            _ => unreachable!(),
        })
        .sum::<isize>();

    let vertical = input
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
        .sum::<isize>() as isize;

    horizontal * vertical
}

#[aoc(day2, part2)]
pub fn solver_2(input: &Movements) -> isize {
    let (mut aim, mut horizontal, mut vertical) = (0isize, 0isize, 0isize);

    input.iter().for_each(|m| match m {
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
    });

    horizontal * vertical
}
