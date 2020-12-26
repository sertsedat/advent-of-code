use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn generate_input(input: &str) -> Vec<i64> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn find_last_spoken_number(numbers: &[i64], target_turn: usize) -> i64 {
    let len = numbers.len();
    let mut last_spokens: HashMap<i64, i64> = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, 1 + i as i64))
        .collect();

    let mut current_number = numbers.last().unwrap().clone();

    for turn in len..target_turn {
        current_number = match last_spokens.entry(current_number) {
            Entry::Vacant(entry) => {
                entry.insert(turn as i64);
                0
            }
            Entry::Occupied(mut entry) => turn as i64 - entry.insert(turn as i64),
        };
    }
    current_number
}

#[aoc(day15, part1)]
pub fn solve_part1(numbers: &[i64]) -> i64 {
    find_last_spoken_number(numbers, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part2(numbers: &[i64]) -> i64 {
    find_last_spoken_number(numbers, 30_000_000)
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
        part1_example_1: ("0,3,6", 436),
        part1_example_2: ("1,3,2", 1),
        part1_example_3: ("2,1,3", 10),
        part1_example_4: ("1,2,3", 27),
        part1_example_5: ("2,3,1", 78),
        part1_example_6: ("3,2,1", 438),
        part1_example_7: ("3,1,2", 1836),
    }

    // macro_rules! test_part2 {
    //     ($($name:ident: $value:expr,)*) => {
    //         $(
    //             #[test]
    //             fn $name() {
    //                 let (input, expected) = $value;
    //                 assert_eq!(solve_part2(&generate_input(input)), expected);
    //             }
    //         )*
    //     }
    // }

    // test_part2! {
    //     part2_example_1: ("0,3,6", 175594),
    //     part2_example_2: ("1,3,2", 2578),
    //     part2_example_3: ("2,1,3", 3544142),
    //     part2_example_4: ("1,2,3", 261214),
    //     part2_example_5: ("2,3,1", 6895259),
    //     part2_example_6: ("3,2,1", 18),
    //     part2_example_7: ("3,1,2", 362),
    // }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day15.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(273, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day15.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(47205, actual);
    }
}
