use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref WRITE_REGEX: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    static ref MASK_REGEX: Regex = Regex::new(r"mask = (.*)").unwrap();
}

pub enum Instruction {
    Mask {
        ones: u64,
        zeroes: u64,
        floating: u64,
    },
    Write {
        address: u64,
        value: u64,
    },
}

#[aoc_generator(day14)]
pub fn generate_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if let Some(memory) = WRITE_REGEX.captures(line) {
                Instruction::Write {
                    address: memory.get(1).unwrap().as_str().parse().unwrap(),
                    value: memory.get(2).unwrap().as_str().parse().unwrap(),
                }
            } else if let Some(mask) = MASK_REGEX.captures(line) {
                let mask = mask.get(1).unwrap().as_str();

                let mut ones: u64 = 0;
                let mut zeroes: u64 = u64::MAX;
                let mut floating: u64 = 0;

                for (i, bitmask) in mask.chars().enumerate() {
                    let digit = 1 << (35 - i);
                    match bitmask {
                        'X' => floating |= digit,
                        '1' => ones |= digit,
                        '0' => zeroes &= !digit,
                        _ => panic!("mask input not valid: {}", mask),
                    }
                }

                Instruction::Mask {
                    ones,
                    zeroes,
                    floating,
                }
            } else {
                panic!("input not valid {}", line);
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> u64 {
    let mut memory = HashMap::new();

    let mut mask_ones = 0;
    let mut mask_zeroes = 0;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask { ones, zeroes, .. } => {
                mask_ones = *ones;
                mask_zeroes = *zeroes;
            }
            Instruction::Write { address, value } => {
                let val_masked = *value & mask_zeroes | mask_ones;
                memory.insert(*address, val_masked);
            }
        }
    }

    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask_ones = 0;
    let mut mask_floating = 0;

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Mask { ones, floating, .. } => {
                mask_ones = *ones;
                mask_floating = *floating;
            }
            Instruction::Write { address, value } => {
                write_to_permutated_addresses(
                    &mut memory,
                    *address | mask_ones,
                    mask_floating,
                    *value,
                );
            }
        }
    }

    memory.values().sum()
}

fn write_to_permutated_addresses(
    memory: &mut HashMap<u64, u64>,
    address: u64,
    floating: u64,
    value: u64,
) {
    for i in 0..1 << floating.count_ones() {
        let mut k = i;
        let mut mask_ones = 0;
        let mut mask_zeros = u64::MAX;
        let mut fb = floating;
        let mut digit = 0;

        while fb != 0 {
            if fb & 1 != 0 {
                if k & 1 == 1 {
                    mask_ones |= 1 << digit;
                } else {
                    mask_zeros &= !(1 << digit);
                }

                k >>= 1;
            }

            fb >>= 1;
            digit += 1;
        }

        memory.insert(address & mask_zeros | mask_ones, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example_part1() {
        let text = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
        let input = generate_input(text);

        let actual = solve_part1(&input);
        assert_eq!(actual, 165);
    }

    #[test]
    fn example_part2() {
        let text = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
        let input = generate_input(text);

        let actual = solve_part2(&input);
        assert_eq!(actual, 208);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day14.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(7997531787333, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day14.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(3564822193820, actual);
    }
}
