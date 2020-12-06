use std::collections::{HashMap, HashSet};

#[aoc_generator(day6)]
pub fn generate_input(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|x| x.to_string()).collect())
        .collect()
}

/// ```
/// use advent_of_code_2020::day6::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day6.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 6782);
/// ```
#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<Vec<String>>) -> usize {
    input
        .iter()
        .map(|group| group.join("").chars().collect::<HashSet<char>>().len())
        .sum()
}

/// ```
/// use advent_of_code_2020::day6::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day6.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 3596);
/// ```
#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Vec<String>>) -> usize {
    input
        .iter()
        .map(|group| {
            let mut yes_answers_counter: HashMap<char, u32> = HashMap::new();
            let group_size = group.len() as u32;

            for person in group {
                for yes_answer in person.chars() {
                    if let Some(a) = yes_answers_counter.get_mut(&yes_answer) {
                        *a += 1;
                    } else {
                        yes_answers_counter.insert(yes_answer, 1);
                    }
                }
            }

            yes_answers_counter
                .values()
                .filter(|&answer| *answer == group_size)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Vec<String>> {
        let text = "abc

a
b
c

ab
ac

a
a
a
a

b";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();
        assert_eq!(solve_part1(&input), 11);
    }

    #[test]
    fn example_part2() {
        let input = get_input();
        assert_eq!(solve_part2(&input), 6);
    }
}
