use std::{collections::HashMap, fmt};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct Board {
    pub numbers: HashMap<usize, (usize, usize)>,
    pub mask: [[bool; 5]; 5],
}

impl Board {
    pub fn new(board: &str) -> Self {
        let mut numbers: HashMap<usize, (usize, usize)> = HashMap::new();
        board.lines().enumerate().for_each(|(row, line)| {
            line.split_whitespace().enumerate().for_each(|(col, num)| {
                numbers.insert(num.parse().unwrap(), (row, col));
            })
        });
        let mask = [[false; 5]; 5];

        Self { numbers, mask }
    }

    pub fn mark(&mut self, number: &usize) -> bool {
        match self.numbers.get(number) {
            Some((row, col)) => {
                self.mask[*row][*col] = true;
                true
            }
            None => false,
        }
    }

    pub fn check(&self) -> bool {
        for row in 0..5 {
            if self.mask[row].into_iter().all(|m| m) {
                return true;
            }
        }

        for col in 0..5 {
            if (0..5).into_iter().map(|row| self.mask[row][col]).all(|m| m) {
                return true;
            }
        }

        false
    }

    pub fn calculate_score(&self) -> usize {
        self.numbers
            .iter()
            .filter(|(_, (row, col))| !self.mask[*row][*col])
            .map(|(number, _)| number)
            .sum::<usize>()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut matrix = [[0; 5]; 5];

        self.numbers
            .iter()
            .for_each(|(number, (row, col))| matrix[*row][*col] = *number);

        let matrix_string = matrix
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|num| format!("{: >3}", num))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mask_string = self
            .mask
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|num| if num { 1 } else { 0 })
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        f.write_str(&format!("{}\n{}\n\n", matrix_string, mask_string))
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub numbers: Vec<usize>,
    pub boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Game {
    let input: Vec<&str> = input.splitn(2, "\n\n").collect();
    let (numbers, boards) = (input[0], input[1]);

    let numbers: Vec<usize> = numbers.split(',').map(|num| num.parse().unwrap()).collect();
    let boards: Vec<Board> = boards
        .split("\n\n")
        .map(|board| Board::new(board))
        .collect();

    Game { numbers, boards }
}

#[aoc(day4, part1)]
pub fn solver_1(game: &Game) -> usize {
    let mut game = game.clone(); // Since cargo-aoc gives us an immutable reference

    for number in game.numbers.iter() {
        // mark the number on all the boards
        game.boards.iter_mut().for_each(|board| {
            board.mark(number);
        });

        // find the first board that wins
        let board_find = game.boards.iter_mut().find(|board| board.check());
        match board_find {
            Some(board) => {
                return board.calculate_score() * number;
            }
            None => {
                continue;
            }
        }
    }

    unreachable!()
}

#[aoc(day4, part2)]
pub fn solver_2(game: &Game) -> usize {
    let mut game = game.clone(); // Since cargo-aoc gives us an immutable reference
    let mut last_score = None;

    for number in game.numbers.iter() {
        // mark the number on all the boards
        for board in game.boards.iter_mut() {
            board.mark(number);
        }

        // find the score of the last board that won
        match game.boards.iter().rfind(|board| board.check()) {
            Some(board) => last_score = Some(board.calculate_score() * number),
            None => (),
        }

        // remove all the boards that won
        game.boards = game
            .boards
            .iter()
            .filter(|board| !board.check())
            .map(|board| board.clone())
            .collect();
    }

    last_score.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_example_1() {
        let game = generator(INPUT);
        let result = solver_1(&game);

        assert_eq!(4512, result);
    }

    #[test]
    fn test_example_2() {
        let game = generator(INPUT);
        let result = solver_2(&game);

        assert_eq!(1924, result);
    }
}
