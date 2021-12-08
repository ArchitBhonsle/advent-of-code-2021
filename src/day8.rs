use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Digit = HashSet<char>;

#[derive(Debug, Clone)]
pub struct Entry {
    signals: [Digit; 10],
    output: [Digit; 4],
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Entry> {
    input
        .trim()
        .split('\n')
        .map(|entry| {
            let entry: Vec<&str> = entry.splitn(2, '|').collect();

            let signals: [Digit; 10] = entry[0]
                .trim()
                .splitn(10, ' ')
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Digit>>()
                .try_into()
                .unwrap();

            let output: [Digit; 4] = entry[1]
                .trim()
                .splitn(10, ' ')
                .map(|s| s.to_string().chars().collect())
                .collect::<Vec<Digit>>()
                .try_into()
                .unwrap();

            Entry { signals, output }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solver_1(entries: &Vec<Entry>) -> usize {
    let check_digit = |digit: &&Digit| {
        let len = digit.len();
        // 1 -> 2, 4 -> 4, 7 -> 3, 8 -> 8
        len == 2 || len == 4 || len == 3 || len == 7
    };

    entries
        .iter()
        .flat_map(|entry| entry.output.iter())
        .filter(check_digit)
        .count()
}

macro_rules! find_and_remove {
    ($vec:ident, $predicate:expr) => {{
        $vec.swap_remove($vec.iter().position($predicate).unwrap())
    }};
}

fn digit_diff(a: &Digit, b: &Digit) -> Digit {
    a.difference(&b).cloned().collect()
}

fn digit_union(a: &Digit, b: &Digit) -> Digit {
    a.union(&b).cloned().collect()
}

fn resolve_signals(signals: &[Digit; 10]) -> [Digit; 10] {
    // order we solve this in:
    // 1, 4, 7, 8: using lengths
    // Create the four pieces
    //     top     : 7 - 1
    //     top_l   : 4 - 1
    //     bottom_l: 8 - (4 | 7)
    //     e       : 8 - 1
    // 9: (remaining - eight) in bottom_l -> bottom_left_edge
    // 6: (e is subset of remaining)
    // 2: (remaining - e).len() == 1 -> top_one -> bottom_one
    // 0: (remaining != 6).len() == 6
    // 4: remaining with 1
    // 5: remaining

    let mut signals: Vec<Digit> = signals.iter().cloned().collect();

    let one = find_and_remove!(signals, |d| d.len() == 2);
    let four = find_and_remove!(signals, |d| d.len() == 4);
    let seven = find_and_remove!(signals, |d| d.len() == 3);
    let eight = find_and_remove!(signals, |d| d.len() == 7);

    let bottom_l = digit_diff(&eight, &digit_union(&four, &seven));
    let e = digit_diff(&eight, &one);

    let nine = find_and_remove!(signals, |d| digit_union(&bottom_l, &digit_diff(&eight, &d))
        .len()
        == 2);
    let bottom_left_edge = digit_diff(&eight, &nine).into_iter().next().unwrap();
    let six = find_and_remove!(signals, |d| e.is_subset(&d));
    let zero = find_and_remove!(signals, |d| d.len() == 6);
    let two = find_and_remove!(signals, |d| d.contains(&bottom_left_edge));
    let five = find_and_remove!(signals, |d| digit_diff(&d, &one).len() == 4);
    let three = signals.into_iter().next().unwrap();

    [zero, one, two, three, four, five, six, seven, eight, nine]
}

#[aoc(day8, part2)]
pub fn solver_2(entries: &Vec<Entry>) -> usize {
    entries
        .into_iter()
        .map(|entry| {
            let resolved = resolve_signals(&entry.signals);
            let output: Vec<usize> = entry
                .output
                .iter()
                .map(|o| resolved.iter().position(|r| r.eq(&o)).unwrap())
                .collect();

            output[0] * 1000 + output[1] * 100 + output[2] * 10 + output[3]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_example_1() {
        let entries = generator(INPUT);
        let result = solver_1(&entries);

        assert_eq!(result, 26);
    }

    #[test]
    fn test_example_2() {
        let entries = generator(INPUT);
        let result = solver_2(&entries);

        assert_eq!(result, 61229);
    }
}
