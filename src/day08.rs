use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Accumulate(i32),
    Jump(i32),
    Noop(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let instruction: Vec<&str> = instruction.split_whitespace().collect();
        let operation = instruction[0];
        let argument = instruction[1];

        let argument = match argument.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(()),
        };

        Ok(match operation {
            "acc" => Instruction::Accumulate(argument),
            "jmp" => Instruction::Jump(argument),
            "nop" => Instruction::Noop(argument),
            _ => return Err(()),
        })
    }
}

pub struct GameConsole {
    pub accumulated: i32,
    pc: u32,
    instruction_stack: Vec<Instruction>,
    processed: HashSet<u32>,
}

impl GameConsole {
    fn new() -> GameConsole {
        GameConsole {
            pc: 0,
            accumulated: 0,
            instruction_stack: Vec::new(),
            processed: HashSet::new(),
        }
    }
    fn execute(&mut self, counter: u32, instruction: &Instruction) -> Option<u32> {
        if let Some(_) = self.processed.get(&counter) {
            return None;
        }

        let (acc, pc) = match instruction {
            Instruction::Accumulate(argument) => (self.accumulated + argument, counter + 1),
            Instruction::Jump(argument) => (self.accumulated, (counter as i32 + argument) as u32),
            Instruction::Noop(_) => (self.accumulated, counter + 1),
        };
        self.accumulated = acc;
        self.pc = counter;

        self.processed.insert(self.pc);
        self.instruction_stack.push(instruction.clone());

        Some(pc)
    }
}

#[aoc_generator(day08)]
pub fn generate_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day08, part1)]
pub fn solve_part1(instructions: &Vec<Instruction>) -> i32 {
    let length: i32 = instructions.len() as i32;
    let mut i: i32 = 0;
    let mut console = GameConsole::new();

    while i < length {
        let instruction = &instructions[i as usize];
        if let Some(next) = console.execute(i as u32, instruction) {
            i = next as i32;
        } else {
            break;
        }
    }
    console.accumulated
}

#[aoc(day08, part2)]
pub fn solve_part2(instructions: &Vec<Instruction>) -> i32 {
    let length: i32 = instructions.len() as i32;

    'outer: for n in 0..length {
        let mut swapped: Vec<Instruction> = instructions.clone();
        let _old: Vec<_> = swapped
            .splice(
                n as usize..(n + 1) as usize,
                vec![match swapped[n as usize] {
                    Instruction::Accumulate(v) => Instruction::Accumulate(v),
                    Instruction::Noop(v) => Instruction::Jump(v),
                    Instruction::Jump(v) => Instruction::Noop(v),
                }],
            )
            .collect();

        let mut i: i32 = 0;
        let mut console = GameConsole::new();
        while i < length {
            let instruction = &swapped[i as usize];
            if let Some(next) = console.execute(i as u32, instruction) {
                i = next as i32;
            } else {
                continue 'outer;
            }
        }
        return console.accumulated;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input() -> Vec<Instruction> {
        let text = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 5)
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 8)
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day8.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(1384, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day8.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(761, actual);
    }
}
