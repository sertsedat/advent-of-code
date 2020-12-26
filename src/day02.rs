use std::ops::RangeInclusive;

pub struct PasswordPolicy {
    letter: char,
    bounds: RangeInclusive<usize>,
    password: String,
}

#[aoc_generator(day02)]
pub fn generate_input(input: &str) -> Vec<PasswordPolicy> {
    input
        .lines()
        .filter_map(|line: &str| {
            if let [bounds, letter, password] = line.split_whitespace().collect::<Vec<&str>>()[..3]
            {
                let bounds: Vec<usize> = bounds.split("-").map(|x| x.parse().unwrap()).collect();
                let bounds = RangeInclusive::new(bounds[0], bounds[1]);

                let letter: char = letter.chars().next().unwrap();

                Some(PasswordPolicy {
                    bounds,
                    letter,
                    password: password.to_owned(),
                })
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day02, part1)]
pub fn solve_part1(input: &Vec<PasswordPolicy>) -> usize {
    input
        .iter()
        .filter(|policy| {
            policy.bounds.contains(
                &policy
                    .password
                    .chars()
                    .filter(|c| c == &policy.letter)
                    .count(),
            )
        })
        .count()
}

#[aoc(day02, part2)]
pub fn solve_part2(input: &Vec<PasswordPolicy>) -> usize {
    input
        .iter()
        .filter(|policy| {
            let (lower, upper) = policy.bounds.clone().into_inner();
            let lower_char = policy.password.chars().nth(lower - 1).unwrap();
            let upper_char = policy.password.chars().nth(upper - 1).unwrap();

            (lower_char == policy.letter) ^ (upper_char == policy.letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> Vec<PasswordPolicy> {
        let text = "1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 1);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day2.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(625, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day2.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(391, actual);
    }
}
