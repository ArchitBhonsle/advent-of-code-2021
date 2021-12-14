use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Cavern {
    octopi: Vec<Vec<usize>>,
}

impl Cavern {
    fn rows(&self) -> usize {
        self.octopi.len()
    }

    fn cols(&self) -> usize {
        self.octopi[0].len()
    }

    fn step(&mut self) -> usize {
        let mut flashes = 0;

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                self.octopi[row][col] += 1;
                self.check(row, col);
            }
        }

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                if self.octopi[row][col] > 9 {
                    flashes += 1;
                    self.octopi[row][col] = 0
                }
            }
        }

        flashes
    }

    fn check(&mut self, row: usize, col: usize) {
        let curr = self.octopi[row][col];

        if curr < 10 {
            return;
        }

        if curr == 10 {
            self.valid_neighbours(row, col).iter().for_each(|(r, c)| {
                self.octopi[*r][*c] += 1;
                self.check(*r, *c);
            });
        }
    }

    fn valid_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let (row, col) = (row as isize, col as isize);
        [-1, 0, 1]
            .into_iter()
            .flat_map(|rd| [-1, 0, 1].into_iter().map(move |cd| (rd, cd)))
            .map(|(rd, cd)| (row + rd, col + cd))
            .filter(|(r, c)| {
                0 <= *r && *r < self.rows() as isize && 0 <= *c && *c < self.cols() as isize
            })
            .map(|(r, c)| (r as usize, c as usize))
            .collect()
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Cavern {
    Cavern {
        octopi: input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|o| o.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    }
}

#[aoc(day11, part1)]
pub fn solver_1(cavern: &Cavern) -> usize {
    let mut cavern = cavern.clone();
    let mut flashes = 0;

    for _ in 1..=100 {
        flashes += cavern.step();
    }

    flashes
}

#[aoc(day11, part2)]
pub fn solver_2(cavern: &Cavern) -> usize {
    let mut cavern = cavern.clone();

    let mut step = 1;
    loop {
        if cavern.step() == cavern.rows() * cavern.cols() {
            return step;
        };
        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn example_1() {
        let cavern = generator(INPUT);
        let result = solver_1(&cavern);

        assert_eq!(result, 1656);
    }

    #[test]
    fn example_2() {
        let cavern = generator(INPUT);
        let result = solver_2(&cavern);

        assert_eq!(result, 195);
    }
}
