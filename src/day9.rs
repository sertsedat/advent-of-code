#[aoc_generator(day9)]
pub fn generate_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn find_noncompliant_number(input: &[u64], preamble: usize) -> u64 {
    let windows = input.windows(preamble);
    let after_preamble = input.iter().skip(preamble);
    for (&target_number, window) in after_preamble.zip(windows) {
        let found_pair = window.iter().enumerate().any(|(i, &current_number)| {
            if target_number > current_number {
                let diff = target_number - current_number;
                return window[(i + 1)..].contains(&diff);
            }
            false
        });

        if !found_pair {
            return target_number;
        }
    }

    0
}

/// ```
/// use advent_of_code_2020::day9::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day9.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 375054920);
/// ```
#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    find_noncompliant_number(input, 25)
}

fn add_min_max(input: &[u64]) -> u64 {
    let mut max = input[0];
    let mut min = input[0];

    for value in input {
        let value = *value;
        if value > max {
            max = value;
        } else if value < min {
            min = value;
        }
    }

    return min + max;
}

/// ```
/// use advent_of_code_2020::day9::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day9.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 54142584);
/// ```
#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let noncompliant_number: u64 = find_noncompliant_number(input, 25);
    find_encryption_weakness(input, noncompliant_number)
}

fn find_encryption_weakness(input: &[u64], noncompliant_number: u64) -> u64 {
    let length = input.len();

    for i in 0..length {
        let mut sum = input[i];
        for j in i + 1..length {
            sum += input[j];
            if sum == noncompliant_number {
                return add_min_max(&input[i..j]);
            }
        }
    }
    0
}
 
#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<u64> {
        let text = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        let actual = find_noncompliant_number(&input, 5);
        assert_eq!(actual, 127)
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        let noncompliant_number = find_noncompliant_number(&input, 5);
        let actual = find_encryption_weakness(&input, noncompliant_number);
        assert_eq!(actual, 62)
    }
}
