use std::collections::{HashMap, HashSet};

#[aoc_generator(day10)]
pub fn generate_input(input: &str) -> Vec<u64> {
    let mut input: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    input.sort();
    input.push(input.last().unwrap() + 3);
    input.insert(0, 0);
    input
}

/// ```
/// use advent_of_code_2020::day10::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day10.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 2812);
/// ```
#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<u64>) -> u64 {
    let mut ones = 0;
    let mut threes = 0;
    for (low, high) in input.iter().zip(input[1..].iter()) {
        let diff = high - low;
        match diff {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

fn total_combinations(
    joltages: &HashSet<u64>,
    jolt: u64,
    device_jolt: u64,
    memo: &mut HashMap<u64, u64>,
) -> u64 {
    if jolt == device_jolt {
        return 1;
    }

    if let Some(c) = memo.get(&jolt) {
        return *c;
    }

    let mut combinations = 0;

    for j in jolt + 1..jolt + 4 {
        if let Some(_) = joltages.get(&j) {
            combinations += total_combinations(joltages, j, device_jolt, memo);
        }
    }

    memo.insert(jolt, combinations);
    combinations
}

/// ```
/// use advent_of_code_2020::day10::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day10.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 386869246296064);
/// ```
#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
    let device_jolt = input[input.len() - 1];
    let joltages: HashSet<u64> = input.iter().map(|&x| x).collect();

    let mut memo: HashMap<u64, u64> = HashMap::new();
    total_combinations(&joltages, 0, device_jolt, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_1() -> Vec<u64> {
        let text = "16
10
15
5
1
11
7
19
6
12
4";
        generate_input(text)
    }

    fn get_input_2() -> Vec<u64> {
        let text = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        generate_input(text)
    }

    #[test]
    fn example_part1_1() {
        let input = get_input_1();

        let actual = solve_part1(&input);
        assert_eq!(actual, 35);
    }

    #[test]
    fn example_part1_2() {
        let input = get_input_2();

        let actual = solve_part1(&input);
        assert_eq!(actual, 220);
    }

    #[test]
    fn example_part2_1() {
        let input = get_input_1();

        let actual = solve_part2(&input);
        assert_eq!(actual, 8);
    }

    #[test]
    fn example_part2_2() {
        let input = get_input_2();

        let actual = solve_part2(&input);
        assert_eq!(actual, 19208);
    }
}
