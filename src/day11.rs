static DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            _ => panic!("input not valid: {}", c),
        }
    }
}

type Layout = Vec<Vec<Seat>>;

#[aoc_generator(day11)]
fn generate_input(input: &str) -> Layout {
    input
        .lines()
        .map(|l| l.chars().map(Seat::from).collect())
        .collect()
}

fn find_final_seating<F>(layout: &Layout, get_next_seat: F) -> u32
where
    F: Fn(&Layout, Seat, (usize, usize)) -> Seat,
{
    let rows = layout.len();
    let cols = layout[0].len();

    let mut layout = layout.clone();

    loop {
        let mut next_layout = Vec::with_capacity(rows);
        let mut has_layout_changed = false;
        for i in 0..rows {
            let mut next_row = Vec::with_capacity(cols);
            for j in 0..cols {
                let seat = layout[i][j];
                if seat == Seat::Floor {
                    next_row.push(seat);
                    continue;
                }

                let next_seat = get_next_seat(&layout, seat, (i, j));

                has_layout_changed |= seat != next_seat;

                next_row.push(next_seat);
            }
            next_layout.push(next_row);
        }

        if !has_layout_changed {
            break;
        }
        layout = next_layout;
    }

    layout
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&s| s == Seat::Occupied)
        .count() as u32
}

/// ```
/// use advent_of_code_2020::day11::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day11.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 2483);
/// ```
#[aoc(day11, part1)]
fn solve_part1(input_layout: &Layout) -> u32 {
    find_final_seating(input_layout, |layout, seat, (row, column)| {
        let total_occupied_adjacent_seats = DIRECTIONS
            .iter()
            .map(|&(r, c)| ((r + row as isize) as usize, (c + column as isize) as usize))
            .filter_map(|(nr, nc)| layout.get(nr).and_then(|row| row.get(nc)))
            .filter(|&&c| c == Seat::Occupied)
            .count();

        match seat {
            Seat::Empty if total_occupied_adjacent_seats == 0 => Seat::Occupied,
            Seat::Occupied if total_occupied_adjacent_seats >= 4 => Seat::Empty,
            _ => seat,
        }
    })
}

fn find_occupied_adjacent_for_direction(
    layout: &Layout,
    (row, column): (usize, usize),
    (dr, dc): (isize, isize),
) -> Option<Seat> {
    let (mut row, mut column) = (row as isize, column as isize);

    loop {
        row += dr;
        column += dc;

        match layout
            .get(row as usize)
            .and_then(|row| row.get(column as usize))
        {
            Some(Seat::Floor) => continue,
            Some(Seat::Occupied) => return Some(Seat::Occupied),
            None | Some(Seat::Empty) => return None,
        }
    }
}

/// ```
/// use advent_of_code_2020::day11::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day11.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 2285);
/// ```
#[aoc(day11, part2)]
fn solve_part2(input_layout: &Layout) -> u32 {
    find_final_seating(input_layout, |layout, seat, (row, column)| {
        let total_occupied_adjacent_seats = DIRECTIONS
            .iter()
            .filter_map(|&(r, c)| {
                find_occupied_adjacent_for_direction(&layout, (row, column), (r, c))
            })
            .count();

        match seat {
            Seat::Empty if total_occupied_adjacent_seats == 0 => Seat::Occupied,
            Seat::Occupied if total_occupied_adjacent_seats >= 5 => Seat::Empty,
            _ => seat,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Layout {
        let text = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();
        let actual = solve_part1(&input);
        assert_eq!(actual, 37);
    }

    #[test]
    fn example_part2() {
        let input = get_input();
        let actual = solve_part2(&input);
        assert_eq!(actual, 26);
    }
}
