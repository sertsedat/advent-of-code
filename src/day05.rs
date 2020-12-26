#[aoc_generator(day05)]
pub fn generate_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| find_seat(line)).collect()
}

struct SeatRange(u32, u32);

impl SeatRange {
    pub fn take_upper(&mut self) {
        self.0 = self.0 + self.diff();
    }

    pub fn take_lower(&mut self) {
        self.1 = self.1 - self.diff();
    }

    fn diff(&self) -> u32 {
        ((self.1 as f32 - self.0 as f32) / 2f32).round() as u32
    }
}

fn find_seat(seat: &str) -> u32 {
    let mut row = SeatRange(0, 127);
    let mut column = SeatRange(0, 7);

    for c in seat.chars() {
        match c {
            'F' => row.take_lower(),
            'B' => row.take_upper(),
            'L' => column.take_lower(),
            'R' => column.take_upper(),
            _ => (),
        }
    }

    row.0 * 8 + column.0
}

#[aoc(day05, part1)]
pub fn solve_part1(seats: &Vec<u32>) -> u32 {
    *seats.iter().max().unwrap()
}

#[aoc(day05, part2)]
pub fn solve_part2(seats: &Vec<u32>) -> u32 {
    let mut seats = seats.clone();
    seats.sort();

    for (seat, next_seat) in seats.iter().zip(seats.iter().skip(1)) {
        if seat + 1 != *next_seat {
            return seat + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    macro_rules! test_part1 {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(solve_part1(&generate_input(input)), expected);
                }
            )*
        }
    }

    test_part1! {
        sample_1: ("FBFBBFFRLR", 357),
        sample_2: ("BFFFBBFRRR", 567),
        sample_3: ("FFFBBBFRRR", 119),
        sample_4: ("BBFFBBFRLL", 820),
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day5.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(991, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day5.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(534, actual);
    }
}
