use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day01)]
pub fn generate_input(input: &str) -> HashSet<u32> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|food| {
                    let amount: u32 = food.parse().unwrap();
                    amount
                })
                .sum()
        })
        .collect()
}

#[aoc(day01, part1)]
pub fn solve_part1(input: &HashSet<u32>) -> u32 {
    *input.into_iter().max().unwrap()
}

#[aoc(day01, part2)]
pub fn solve_part2(input: &HashSet<u32>) -> u32 {
    input.into_iter().sorted_by(|a, b| b.cmp(a)).take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> HashSet<u32> {
        let text = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 24000);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 45000);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2022/day1.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(73211, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2022/day1.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(213958, actual);
    }
}
