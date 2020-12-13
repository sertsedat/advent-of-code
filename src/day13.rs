#[aoc_generator(day13, part1)]
pub fn generate_input_part1(input: &str) -> (u64, Vec<u64>) {
    let mut lines = input.lines();
    let timestamp: u64 = lines.next().unwrap().parse().unwrap();
    let bus_lines: Vec<u64> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&l| l != "x")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    (timestamp, bus_lines)
}

pub struct BusLine(u64, u64);

#[aoc_generator(day13, part2)]
pub fn generate_input_part2(input: &str) -> Vec<BusLine> {
    let mut lines = input.lines();
    lines.next();
    let bus_lines = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, line)| {
            if line == "x" {
                None
            } else {
                Some(BusLine(line.parse().unwrap(), i as u64))
            }
        })
        .collect();
    bus_lines
}

/// ```
/// use advent_of_code_2020::day13::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day13.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input_part1(&input)), 259);
/// ```
#[aoc(day13, part1)]
pub fn solve_part1((timestamp, bus_lines): &(u64, Vec<u64>)) -> u64 {
    let bus_lines = bus_lines;
    let mut t = 0;
    loop {
        let earliest = bus_lines
            .iter()
            .find(|&line| (t + timestamp) % line == 0)
            .map(|line| line * t);

        match earliest {
            Some(time) => return time,
            None => t += 1,
        }
    }
}

/// ```
/// use advent_of_code_2020::day13::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day13.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input_part2(&input)), 210612924879242);
/// ```
#[aoc(day13, part2)]
pub fn solve_part2(bus_lines: &Vec<BusLine>) -> u64 {
    let mut t = 0;
    let mut denominator = 1;

    for BusLine(bus, tick) in bus_lines {
        loop {
            if (t + tick) % bus == 0 {
                denominator *= bus;
                break;
            }
            t += denominator;
        }
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let text = "939
7,13,x,x,59,x,31,19
";
        let input = generate_input_part1(text);

        let actual = solve_part1(&input);
        assert_eq!(actual, 295);
    }

    #[test]
    fn example1_part2() {
        let text = "939
7,13,x,x,59,x,31,19
";
        let input = generate_input_part2(text);

        let actual = solve_part2(&input);
        assert_eq!(actual, 1068781);
    }

    #[test]
    fn example2_part2() {
        let text = "939
17,x,13,19
";
        let input = generate_input_part2(text);

        let actual = solve_part2(&input);
        assert_eq!(actual, 3417);
    }

    #[test]
    fn example3_part2() {
        let text = "939
67,7,59,61
";
        let input = generate_input_part2(text);

        let actual = solve_part2(&input);
        assert_eq!(actual, 754018);
    }
}
