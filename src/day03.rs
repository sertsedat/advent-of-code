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

/// ```
/// use advent_of_code_2020::day03::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day3.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 286);
/// ```
#[aoc(day03, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    count_encountered_trees(input, &3, &1)
}

/// ```
/// use advent_of_code_2020::day03::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day3.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 3638606400);
/// ```
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
}
