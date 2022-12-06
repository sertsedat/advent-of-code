use itertools::Itertools;

#[aoc_generator(day06)]
pub fn generate_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn find_first_marker(input: &Vec<char>, window_size: usize) -> usize {
    input
        .windows(window_size)
        .position(|window| window.iter().unique().count() == window_size)
        .unwrap()
        + window_size
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &Vec<char>) -> usize {
    find_first_marker(input, 4)
}

#[aoc(day06, part2)]
pub fn solve_part2(input: &Vec<char>) -> usize {
    find_first_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<char> {
        generate_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 11);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 26);
    }

    #[test]
    fn test_input_part1() {
        let input = generate_input(include_str!("../../input/2022/day6.txt"));
        let result = solve_part1(&input);

        assert_eq!(result, 1262);
    }

    #[test]
    fn test_input_part2() {
        let input = generate_input(include_str!("../../input/2022/day6.txt"));
        let result = solve_part2(&input);

        assert_eq!(result, 3444);
    }
}
