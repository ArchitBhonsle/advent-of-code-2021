use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct LanternFish {
    days: usize,
}

impl Default for LanternFish {
    fn default() -> Self {
        LanternFish { days: 8 }
    }
}

impl LanternFish {
    fn step(&mut self) -> bool {
        if self.days == 0 {
            self.days = 6;
            return true;
        }
        self.days -= 1;

        return false;
    }

    fn check_step(&self) -> (usize, bool) {
        if self.days == 0 {
            return (6, true);
        }
        return (self.days - 1, false);
    }
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<LanternFish> {
    input
        .trim()
        .split(',')
        .map(|line| line.parse().unwrap())
        .map(|days| LanternFish { days })
        .collect()
}

#[aoc(day6, part1)]
pub fn solver_1(input: &Vec<LanternFish>) -> usize {
    let mut lantern_fish = input.clone();

    (1..=80).for_each(|_| {
        let new_fish = lantern_fish
            .iter_mut()
            .map(|lf| lf.step()) // mutating map bad!
            .filter(|nf| *nf)
            .count();

        lantern_fish.extend((0..new_fish).map(|_| LanternFish::default()));
    });

    lantern_fish.len()
}

#[aoc(day6, part2)]
pub fn solver_2(input: &Vec<LanternFish>) -> usize {
    let mut fish_count: HashMap<LanternFish, usize> = (0..=8)
        .map(|days| LanternFish { days })
        .map(|lf| (lf, 0))
        .collect();

    input
        .iter()
        .for_each(|lf| *fish_count.get_mut(lf).unwrap() += 1);

    (1..=256).for_each(|_| {
        let mut new_fish_count: HashMap<LanternFish, usize> = (0..=8)
            .map(|days| LanternFish { days })
            .map(|lf| (lf, 0))
            .collect();

        fish_count.iter().for_each(|(lf, cnt)| {
            let (new_days, if_reproduce) = lf.check_step();
            *new_fish_count
                .get_mut(&LanternFish { days: new_days })
                .unwrap() += *cnt;

            if if_reproduce {
                *new_fish_count.get_mut(&LanternFish { days: 8 }).unwrap() += *cnt;
            }
        });

        fish_count = new_fish_count;
    });

    fish_count.values().sum()
}

#[aoc(day6, part1, Faster)]
pub fn solver_1_faster(input: &Vec<LanternFish>) -> usize {
    let mut fish_count: HashMap<LanternFish, usize> = (0..=8)
        .map(|days| LanternFish { days })
        .map(|lf| (lf, 0))
        .collect();

    input
        .iter()
        .for_each(|lf| *fish_count.get_mut(lf).unwrap() += 1);

    (1..=80).for_each(|_| {
        let mut new_fish_count: HashMap<LanternFish, usize> = (0..=8)
            .map(|days| LanternFish { days })
            .map(|lf| (lf, 0))
            .collect();

        fish_count.iter().for_each(|(lf, cnt)| {
            let (new_days, if_reproduce) = lf.check_step();
            *new_fish_count
                .get_mut(&LanternFish { days: new_days })
                .unwrap() += *cnt;

            if if_reproduce {
                *new_fish_count.get_mut(&LanternFish { days: 8 }).unwrap() += *cnt;
            }
        });

        fish_count = new_fish_count;
    });

    fish_count.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn example_1() {
        let input = generator(INPUT);
        let result = solver_1(&input);

        assert_eq!(result, 5934);
    }

    #[test]
    fn example_1_faster() {
        let input = generator(INPUT);
        let result = solver_1_faster(&input);

        assert_eq!(result, 5934);
    }

    #[test]
    fn example_2() {
        let input = generator(INPUT);
        let result = solver_2(&input);

        assert_eq!(result, 26984457539);
    }
}
