#[derive(Clone, Copy)]
pub enum Cell {
    Inactive = 0,
    Active = 1,
}

struct Grid {
    size: u8,
    dimension: u8,
    cells: Vec<Cell>,
}

impl Grid {
    fn new(input: &[Cell], n: u8, dimension: u8) -> Grid {
        assert!(dimension == 3 || dimension == 4);

        let size = 18;
        let cells = (0..i32::pow(size as i32, dimension as u32))
            .map(|_| Cell::Inactive)
            .collect();
        let mut grid = Grid {
            size,
            cells,
            dimension,
        };

        for y in 0..n {
            for x in 0..n {
                let index_input = (x as u16 + n as u16 * y as u16) as usize;
                let offset = (size - n) / 2;
                let index_grid =
                    grid.get_index(offset + x as u8, offset + y as u8, size / 2, Some(size / 2));

                grid.cells[index_grid] = input[index_input];
            }
        }

        grid
    }

    fn out_of_bounds(&self, i: i8) -> bool {
        i.is_negative() || i as u8 >= self.size
    }

    fn run_six_cycles(&mut self) {
        for _ in 0..6 {
            self.run_cycle();
        }
    }

    fn run_cycle(&mut self) {
        let mut next = self.cells.clone();

        for z in 0..self.size {
            for y in 0..self.size {
                for x in 0..self.size {
                    for w in 0..self.size {
                        let index = self.get_index(x, y, z, Some(w));
                        let cell = self.cells[index];
                        let occupied_neighbors = self.count_occupied_neighbors(x, y, z, w);

                        let next_cell = match (cell, occupied_neighbors) {
                            (Cell::Active, 2..=3) => Cell::Active,
                            (Cell::Active, _) => Cell::Inactive,

                            (Cell::Inactive, 3) => Cell::Active,
                            (Cell::Inactive, _) => Cell::Inactive,
                        };

                        next[index] = next_cell;
                    }
                }
            }
        }

        self.cells = next;
    }

    fn get_index(&self, x: u8, y: u8, z: u8, w: Option<u8>) -> usize {
        let w = match self.dimension {
            3 => 0,
            4 => w.unwrap(),
            _ => unreachable!(),
        } as usize;
        let size = self.size as usize;
        x as usize + (size * (y as usize + (size * (z as usize + (size * w)))))
    }

    fn count_occupied_neighbors(&self, x: u8, y: u8, z: u8, w: u8) -> u8 {
        let mut count = 0;

        for dz in -1..=1 {
            let nz = z as i8 + dz;
            if self.out_of_bounds(nz) {
                continue;
            }

            for dy in -1..=1 {
                let ny = y as i8 + dy;
                if self.out_of_bounds(ny) {
                    continue;
                }

                for dx in -1..=1 {
                    let nx = x as i8 + dx;
                    if self.out_of_bounds(nx) {
                        continue;
                    }

                    let nx = nx as u8;
                    let ny = ny as u8;
                    let nz = nz as u8;

                    if self.dimension == 3 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }

                        let index = self.get_index(nx, ny, nz, None);
                        count += self.cells[index] as u8;
                    } else {
                        for dw in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }

                            let nw = w as i8 + dw;
                            if self.out_of_bounds(nw) {
                                continue;
                            }

                            let index = self.get_index(nx, ny, nz, Some(nw as u8));
                            count += self.cells[index] as u8;
                        }
                    }
                }
            }
        }

        count
    }

    fn count_active(&self) -> u16 {
        self.cells.iter().map(|&cell| cell as u16).sum()
    }
}

#[aoc_generator(day17)]
pub fn generate_input(input: &str) -> (u8, Vec<Cell>) {
    (
        input.lines().count() as u8,
        input
            .replace("\n", "")
            .chars()
            .map(|c| match c {
                '#' => Cell::Active,
                _ => Cell::Inactive,
            })
            .collect::<Vec<Cell>>(),
    )
}

/// ```
/// use advent_of_code_2020::day17::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day17.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 202);
/// ```
#[aoc(day17, part1)]
pub fn solve_part1((n, input): &(u8, Vec<Cell>)) -> u16 {
    let mut grid = Grid::new(input, *n, 3);
    grid.run_six_cycles();
    grid.count_active()
}

/// ```
/// use advent_of_code_2020::day17::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day17.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 2028);
/// ```
#[aoc(day17, part2)]
pub fn solve_part2((n, input): &(u8, Vec<Cell>)) -> u16 {
    let mut grid = Grid::new(input, *n, 4);
    grid.run_six_cycles();
    grid.count_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (u8, Vec<Cell>) {
        let text = ".#.
..#
###
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();
        let actual = solve_part1(&input);
        assert_eq!(actual, 112);
    }

    #[test]
    fn example_part2() {
        let input = get_input();
        let actual = solve_part2(&input);
        assert_eq!(actual, 848);
    }
}
