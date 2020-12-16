use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref BAG_RULE_REGEX: Regex = Regex::new(r"(\d) ([^,.]*) bag").unwrap();
}

fn for_each_capture<F>(input: &str, mut f: F)
where
    F: FnMut(&str, String, usize),
{
    for line in input.lines() {
        if let Some((parent_color, rest)) = get_color_and_rest(line) {
            for capture in BAG_RULE_REGEX.captures_iter(&rest) {
                let amount = capture[1].to_string().parse().unwrap();
                let child_color = capture[2].to_string();

                f(parent_color, child_color, amount);
            }
        }
    }
}

fn get_color_and_rest(line: &str) -> Option<(&str, &str)> {
    if let [color, rest] = line.split(" bags contain ").collect::<Vec<&str>>()[..2] {
        Some((&color, &rest))
    } else {
        None
    }
}

#[aoc_generator(day07, part1)]
pub fn generate_input_part1(input: &str) -> HashMap<String, HashSet<String>> {
    let mut bags_in: HashMap<String, HashSet<String>> = HashMap::new();

    for_each_capture(input, |parent_color, child_color, _amount| {
        if let Some(child_rule) = bags_in.get_mut(&child_color) {
            child_rule.insert(parent_color.to_string());
        } else {
            let mut child_rule = HashSet::new();
            child_rule.insert(parent_color.clone().to_string());
            bags_in.insert(child_color, child_rule);
        }
    });

    bags_in
}

/// ```
/// use advent_of_code_2020::day07::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day7.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input_part1(&input)), 372);
/// ```
#[aoc(day07, part1)]
pub fn solve_part1(input: &HashMap<String, HashSet<String>>) -> usize {
    let mut containing_bags: HashSet<String> = HashSet::new();

    all_containing_colors(input, "shiny gold", &mut containing_bags);

    containing_bags.len()
}

pub fn all_containing_colors(
    input: &HashMap<String, HashSet<String>>,
    color: &str,
    containing_bags: &mut HashSet<String>,
) {
    if let Some(parents) = input.get(color) {
        for color in parents {
            containing_bags.insert(color.to_string());

            all_containing_colors(input, color, containing_bags);
        }
    }
}

#[aoc_generator(day07, part2)]
pub fn generate_input_part2(input: &str) -> HashMap<String, HashMap<String, usize>> {
    let mut bags_contain: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for_each_capture(input, |parent_color, child_color, amount| {
        if let Some(contain) = bags_contain.get_mut(parent_color) {
            contain.insert(child_color, amount);
        } else {
            let mut contain = HashMap::new();
            contain.insert(child_color, amount);
            bags_contain.insert(parent_color.clone().to_string(), contain);
        }
    });

    bags_contain
}

/// ```
/// use advent_of_code_2020::day07::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day7.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input_part2(&input)), 8015);
/// ```
#[aoc(day07, part2)]
pub fn solve_part2(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    count_total_bags(input, "shiny gold")
}

fn count_total_bags(input: &HashMap<String, HashMap<String, usize>>, color: &str) -> usize {
    let mut total = 0;

    if let Some(rule) = input.get(color) {
        for (color, amount) in rule {
            total += amount + (amount * count_total_bags(input, color));
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = generate_input_part1(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );

        let actual = solve_part1(&input);
        assert_eq!(actual, 4);
    }

    #[test]
    fn example_part2_1() {
        let input = generate_input_part2(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.",
        );

        let actual = solve_part2(&input);
        assert_eq!(actual, 32)
    }

    #[test]
    fn example_part2_2() {
        let input = generate_input_part2(
            "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
",
        );

        let actual = solve_part2(&input);
        assert_eq!(actual, 126);
    }
}
