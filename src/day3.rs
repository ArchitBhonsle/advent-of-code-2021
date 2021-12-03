use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
pub fn generator_1(input: &str) -> (Vec<String>, usize) {
    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let width = input[0].trim().len();

    (input, width)
}

#[aoc(day3, part1)]
pub fn solver_1(input: &(Vec<String>, usize)) -> usize {
    let (input, width) = input;

    let mut count = vec![(0, 0); *width];
    for num in input.iter() {
        for (ind, bit) in num.char_indices() {
            if bit == '0' {
                count[ind].0 += 1;
            } else {
                count[ind].1 += 1;
            }
        }
    }

    let (mut gamma, mut epsilon) = (String::new(), String::new());
    for (zeros, ones) in count {
        if ones > zeros {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    let (gamma, epsilon) = (
        usize::from_str_radix(&gamma, 2).unwrap(),
        usize::from_str_radix(&epsilon, 2).unwrap(),
    );

    gamma * epsilon
}

#[aoc_generator(day3, part2)]
pub fn generator_2(input: &str) -> (Vec<usize>, usize) {
    let width = input.lines().map(|line| line.len()).max().unwrap();
    let input: Vec<usize> = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect();

    (input, width)
}

fn nth_bit(num: &usize, n: usize, width: usize) -> bool {
    num & (1 << (width - n - 1)) != 0
}

fn most_common_nth_bit(nums: &Vec<usize>, n: usize, width: usize) -> bool {
    let zeros = nums.iter().filter(|num| !nth_bit(num, n, width)).count();
    let ones = nums.iter().filter(|num| nth_bit(num, n, width)).count();

    ones >= zeros
}

#[aoc(day3, part2)]
pub fn solver_2(input: &(Vec<usize>, usize)) -> usize {
    let (input, width) = input;

    let mut oxygen_generator = input.clone();
    for n in 0..*width {
        let mcnb = most_common_nth_bit(&oxygen_generator, n, *width);

        oxygen_generator = oxygen_generator
            .into_iter()
            .filter(|num| nth_bit(num, n, *width) == mcnb)
            .collect();

        if oxygen_generator.len() == 1 {
            break;
        }
    }
    let oxygen_generator = oxygen_generator[0];

    let mut co2_scrubber = input.clone();
    for n in 0..*width {
        let mcnb = most_common_nth_bit(&co2_scrubber, n, *width);
        co2_scrubber = co2_scrubber
            .into_iter()
            .filter(|num| nth_bit(num, n, *width) != mcnb)
            .collect();

        if co2_scrubber.len() == 1 {
            break;
        }
    }
    let co2_scrubber = co2_scrubber[0];

    oxygen_generator * co2_scrubber
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nth_bit() {
        let width = 8;
        assert!(nth_bit(&0b0001_0000, 3, width));
        assert!(nth_bit(&0b0100_0000, 1, width));
        assert!(nth_bit(&0b0000_0010, 6, width));
        assert!(nth_bit(&0b0000_1000, 4, width));
    }

    #[test]
    fn test_example() {
        let example = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let result = solver_2(&generator_2(example));

        assert_eq!(result, 230);
    }
}
