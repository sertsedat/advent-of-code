use itertools::Itertools;

#[aoc_generator(day06)]
pub fn generate_input(input: &str) -> Vec<char> {
    input.chars().collect_vec()
}

fn find_first_marker(input: &Vec<char>, window_size: usize) -> usize {
    let (i, _) = input
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| window.iter().unique().count() == window_size)
        .unwrap();
    i + window_size
}

#[aoc(day06, part1)]
pub fn solve_part1(input: &Vec<char>) -> usize {
    let window_size = 4;

    find_first_marker(input, window_size)
}

#[aoc(day06, part2)]
pub fn solve_part2(input: &Vec<char>) -> usize {
    let window_size = 14;

    find_first_marker(input, window_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
        let text = fs::read_to_string("input/2022/day6.txt").unwrap();
        let input = generate_input(&text);
        let result = solve_part1(&input);
        assert_eq!(result, 1262);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2022/day6.txt").unwrap();
        let input = generate_input(&text);
        let result = solve_part2(&input);
        assert_eq!(result, 3444);
    }
}
