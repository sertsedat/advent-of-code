#[aoc_generator(day03)]
pub fn generate_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

pub fn count_encountered_trees(input: &Vec<String>, col_step: &usize, row_step: &usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut found_trees = 0;
    let total_rows = input.len();

    if total_rows == 0 {
        return 0;
    }

    let total_cols = input[0].len();

    while row < total_rows {
        if input[row].as_bytes()[col % total_cols] == b'#' {
            found_trees += 1;
        }

        row += row_step;
        col += col_step;
    }

    found_trees
}

#[aoc(day03, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    count_encountered_trees(input, &3, &1)
}

#[aoc(day03, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().fold(1, |acc, (col_step, row_step)| {
        acc * count_encountered_trees(input, col_step, row_step)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> Vec<String> {
        let text = "..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#";

        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 336);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day3.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(286, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day3.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(3638606400, actual);
    }
}
