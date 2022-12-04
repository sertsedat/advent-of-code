use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day03)]
pub fn generate_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|rucksack| rucksack.chars().collect())
        .collect()
}

fn value_of_item(item: &char) -> u32 {
    match item {
        'a'..='z' => *item as u32 - b'a' as u32 + 1,
        'A'..='Z' => *item as u32 - b'A' as u32 + 27,
        _ => panic!("corrupt intput"),
    }
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

            let unique_in_compartment1: HashSet<&char> = compartment1.iter().collect();
            let unique_in_compartment2: HashSet<&char> = compartment2.iter().collect();

            unique_in_compartment1
                .intersection(&unique_in_compartment2)
                .map(|item| value_of_item(*item))
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> u32 {
    let chunk_size = 3;
    input
        .iter()
        .chunks(chunk_size) //
        .into_iter()
        .map(|group| group.map(|r| r.iter().collect::<HashSet<&char>>()))
        .map(|group| {
            let group: Vec<HashSet<&char>> = group.collect();
            let rucksack1 = &group[0];
            let rucksack2 = &group[1];
            let rucksack3 = &group[2];

            let intersection: HashSet<&char> = rucksack1
                .intersection(rucksack2)
                .map(|c| *c)
                .collect::<HashSet<&char>>();
            intersection
                .intersection(rucksack3)
                .map(|item| value_of_item(item))
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> Vec<Vec<char>> {
        let text = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 157);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 70);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2022/day3.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(8085, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2022/day3.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(2515, actual);
    }
}
