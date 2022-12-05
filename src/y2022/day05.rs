use itertools::Itertools;

#[derive(Debug)]
pub struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let instruction_numbers: Vec<usize> = s
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|n| n.parse().unwrap())
            .collect();

        Instruction {
            amount: instruction_numbers[0],
            from: instruction_numbers[1] - 1,
            to: instruction_numbers[2] - 1,
        }
    }
}

#[aoc_generator(day05)]
pub fn generate_input(input: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (drawing, procedure) = input.split_once("\n\n").unwrap();

    let first_line = drawing.lines().take(1).exactly_one().unwrap();
    let stack_count = (first_line.len() / 4) + 1;
    let mut crates: Vec<Vec<char>> = Vec::with_capacity(stack_count);

    let highest = drawing.lines().count() - 1;
    for _ in 0..stack_count {
        crates.push(Vec::with_capacity(highest));
    }

    drawing.lines().for_each(|line| {
        line.chars()
            .chunks(4)
            .into_iter()
            .map(|chunk| {
                let chunk: Vec<char> = chunk.collect();

                if chunk[0] == '[' {
                    Some(chunk[1])
                } else {
                    None
                }
            })
            .enumerate()
            .for_each(|(i, c)| {
                if let Some(crate_id) = c {
                    if crates[i].len() == 0 {
                        crates[i].push(crate_id)
                    } else {
                        crates[i].splice(0..0, vec![crate_id]);
                    }
                }
            })
    });

    let instructions: Vec<Instruction> = procedure.lines().map(Instruction::from).collect();

    (crates, instructions)
}

fn top_crates(crates: &Vec<Vec<char>>) -> String {
    crates
        .iter()
        .map(|c| c.last())
        .filter(|c| c.is_some())
        .map(|c| *c.unwrap())
        .join("")
        .to_string()
}

#[aoc(day05, part1)]
pub fn solve_part1((crates, instructions): &(Vec<Vec<char>>, Vec<Instruction>)) -> String {
    let mut crates: Vec<Vec<char>> = crates.clone();

    instructions.iter().for_each(|instruction| {
        for _ in 0..instruction.amount {
            let crate_to_move = crates[instruction.from]
                .pop()
                .expect("input wasn't parsed correctly");
            crates[instruction.to].push(crate_to_move);
        }
    });

    top_crates(&crates)
}

#[aoc(day05, part2)]
pub fn solve_part2((crates, instructions): &(Vec<Vec<char>>, Vec<Instruction>)) -> String {
    let mut crates: Vec<Vec<char>> = crates.clone();

    instructions.iter().for_each(|instruction| {
        let from_crate = &crates[instruction.from];
        let remove_range = (from_crate.len() - instruction.amount)..from_crate.len();

        let mut crates_to_move: Vec<char> = crates[instruction.from].drain(remove_range).collect();
        crates[instruction.to].append(&mut crates_to_move);
    });

    top_crates(&crates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> (Vec<Vec<char>>, Vec<Instruction>) {
        let text = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), "CMZ");
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), "MCD");
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2022/day5.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!("PTWLTDSJV", actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2022/day5.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!("WZMFVGGZP", actual);
    }
}
