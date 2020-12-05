use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn generate_input(input: &str) -> HashSet<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/// ```
/// use advent_of_code_2020::day01::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day1.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 864864);
/// ```
#[aoc(day1, part1)]
pub fn solve_part1(input: &HashSet<u32>) -> u32 {
    let target = 2020;

    for x in input {
        let y = target - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

/// ```
/// use advent_of_code_2020::day01::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day1.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 281473080);
/// ```
#[aoc(day1, part2)]
pub fn solve_part2(input: &HashSet<u32>) -> u32 {
    let target = 2020;

    for x in input {
        for y in input {
            if x + y > target {
                continue;
            }

            let z = target - x - y;

            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> HashSet<u32> {
        let text = "1721
979
366
299
675
1456
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 514579);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 241861950);
    }
}
