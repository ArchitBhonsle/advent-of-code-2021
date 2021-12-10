use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn is_opening(bracket: char) -> bool {
    match bracket {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn complement(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn checker_score(closing: char) -> usize {
    match closing {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[aoc(day10, part1)]
pub fn solver_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut stack = Vec::new();

            let mut line_score = 0;
            for bracket in line.chars() {
                if is_opening(bracket) {
                    stack.push(bracket);
                } else {
                    if bracket == complement(*stack.last().unwrap()) {
                        stack.pop().unwrap();
                    } else {
                        line_score = checker_score(bracket);
                        break;
                    }
                }
            }

            line_score
        })
        .sum()
}

fn complete_scores(bracket: char) -> usize {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

#[aoc(day10, part2)]
pub fn solver_2(input: &Vec<String>) -> usize {
    let completion_scores: Vec<usize> = input
        .iter()
        .map(|line| {
            let mut stack = Vec::new();

            let mut incorrect = false;
            for bracket in line.chars() {
                if is_opening(bracket) {
                    stack.push(bracket);
                } else {
                    if bracket == complement(*stack.last().unwrap()) {
                        stack.pop().unwrap();
                    } else {
                        incorrect = true;
                        break;
                    }
                }
            }

            (stack, incorrect)
        })
        .filter(|(_, incorrect)| !incorrect)
        .map(|(stack, _)| {
            stack
                .iter()
                .rev()
                .fold(0, |acc, x| acc * 5 + complete_scores(*x))
        })
        .sorted()
        .collect();

    completion_scores[completion_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn example_1() {
        let input = generator(INPUT);
        let result = solver_1(&input);

        assert_eq!(result, 26397);
    }

    #[test]
    fn example_2() {
        let input = generator(INPUT);
        let result = solver_2(&input);

        assert_eq!(result, 288957);
    }
}
