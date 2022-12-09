use itertools::Itertools;

#[derive(Debug)]
struct Tree(u32);

impl From<char> for Tree {
    fn from(c: char) -> Self {
        Self(c.to_digit(10).unwrap())
    }
}

#[derive(Debug)]
pub struct Grid {
    trees: Vec<Vec<Tree>>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u32 {
        self.trees[x][y].0
    }

    fn is_tree_visible_from_right(&self, x: usize, y: usize, height: u32) -> bool {
        ((x + 1)..self.len())
            .into_iter()
            .all(|i| self.get(i, y) < height)
    }

    fn is_tree_visible_from_left(&self, x: usize, y: usize, height: u32) -> bool {
        (0..x).into_iter().all(|i| self.get(i, y) < height)
    }

    fn is_tree_visible_from_top(&self, x: usize, y: usize, height: u32) -> bool {
        (0..y).into_iter().all(|j| self.get(x, j) < height)
    }

    fn is_tree_visible_from_bottom(&self, x: usize, y: usize, height: u32) -> bool {
        ((y + 1)..self.len())
            .into_iter()
            .all(|j| self.get(x, j) < height)
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.len() - 1 || y == self.len() - 1 {
            return true;
        }

        let height = self.get(x, y);

        self.is_tree_visible_from_bottom(x, y, height)
            || self.is_tree_visible_from_left(x, y, height)
            || self.is_tree_visible_from_right(x, y, height)
            || self.is_tree_visible_from_top(x, y, height)
    }

    fn scenic_score_for_tree_up(&self, x: usize, y: usize, height: u32) -> usize {
        let score = (0..x)
            .rev()
            .take_while(|&i| self.get(i, y) < height)
            .count();

        if score != x {
            score + 1
        } else {
            score
        }
    }

    fn scenic_score_for_tree_down(&self, x: usize, y: usize, height: u32) -> usize {
        let score = ((x + 1)..self.len())
            .take_while(|&i| self.get(i, y) < height)
            .count();

        if score != self.len() - x - 1 {
            score + 1
        } else {
            score
        }
    }
    fn scenic_score_for_tree_left(&self, x: usize, y: usize, height: u32) -> usize {
        let score = (0..y)
            .rev()
            .take_while(|&j| self.get(x, j) < height)
            .count();

        if score != y {
            score + 1
        } else {
            score
        }
    }

    fn scenic_score_for_tree_right(&self, x: usize, y: usize, height: u32) -> usize {
        let score = ((y + 1)..self.len())
            .take_while(|&j| self.get(x, j) < height)
            .count();

        if score != self.len() - y - 1 {
            score + 1
        } else {
            score
        }
    }

    fn scenic_score_for_tree(&self, x: usize, y: usize) -> usize {
        let height = self.get(x, y);

        self.scenic_score_for_tree_left(x, y, height)
            * self.scenic_score_for_tree_right(x, y, height)
            * self.scenic_score_for_tree_up(x, y, height)
            * self.scenic_score_for_tree_down(x, y, height)
    }

    fn len(&self) -> usize {
        self.trees.len()
    }
}

#[aoc_generator(day08)]
pub fn generate_input(input: &str) -> Grid {
    Grid {
        trees: input
            .lines()
            .map(|l| l.chars().map(Tree::from).collect_vec())
            .collect_vec(),
    }
}

fn permutate_indices(max: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..max)
        .zip(0..max)
        .chain((0..max).permutations(2).map(|v| (v[0], v[1])))
        .sorted()
}

#[aoc(day08, part1)]
pub fn solve_part1(input: &Grid) -> usize {
    permutate_indices(input.len())
        .filter(|(x, y)| input.is_tree_visible(*x, *y))
        .count()
}

#[aoc(day08, part2)]
pub fn solve_part2(input: &Grid) -> usize {
    permutate_indices(input.len())
        .map(|(x, y)| input.scenic_score_for_tree(x, y))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Grid {
        generate_input(
            "30373
25512
65332
33549
35390",
        )
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 21);
    }

    #[test]
    fn test_input_part1() {
        let input = generate_input(include_str!("../../input/2022/day8.txt"));
        let result = solve_part1(&input);

        assert_eq!(result, 1851);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 8);
    }

    #[test]
    fn test_input_part2() {
        let input = generate_input(include_str!("../../input/2022/day8.txt"));
        let result = solve_part2(&input);

        assert_eq!(result, 574080);
    }
}
