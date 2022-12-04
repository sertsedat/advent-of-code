use std::{ops::RangeInclusive};

#[aoc_generator(day04)]
pub fn generate_input(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|pairs| {
            let (elf1, elf2) = pairs.split_once(",").unwrap();
            let (elf1_start, elf1_end) = elf1.split_once("-").unwrap();
            let (elf2_start, elf2_end) = elf2.split_once("-").unwrap();

            (
                elf1_start.parse().unwrap()..=elf1_end.parse().unwrap(),
                elf2_start.parse().unwrap()..=elf2_end.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day04, part1)]
pub fn solve_part1(input: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| {
            (elf1.start() <= elf2.start() && elf1.end() >= elf2.end())
                || (elf2.start() <= elf1.start() && elf2.end() >= elf1.end())
        })
        .count()
}

#[aoc(day04, part2)]
pub fn solve_part2(input: &Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> usize {
    input
        .iter()
        .filter(|(elf1, elf2)| !(elf1.end() < elf2.start() || elf1.start() > elf2.end()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
        let text = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
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

        assert_eq!(solve_part2(&input), 4);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2022/day4.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(462, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2022/day4.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(835, actual);
    }
}
