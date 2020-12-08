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
    tried_converting: HashSet<u32>,
}

impl GameConsole {
    fn new() -> GameConsole {
        GameConsole {
            pc: 0,
            accumulated: 0,
            instruction_stack: Vec::new(),
            processed: HashSet::new(),
            tried_converting: HashSet::new(),
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
    fn revert_to_last_breakpoint(&mut self) -> Option<u32> {
        loop {
            let length = self.instruction_stack.len();

            if length < 2 {
                self.accumulated = 0;
                self.pc = 0;
                self.tried_converting.insert(0);
                self.instruction_stack.pop();
                self.processed = HashSet::new();
                break;
            }

            let curr_ins = self.instruction_stack[length - 1];
            let prev_ins = self.instruction_stack[length - 2];

            let accumulation = match &curr_ins {
                Instruction::Accumulate(argument) => self.accumulated - argument,
                _ => self.accumulated,
            };

            let pc = match &prev_ins {
                Instruction::Jump(argument) => (self.pc as i32 - argument) as u32,
                _ => self.pc - 1,
            };

            let prev_pc = self.pc;

            self.pc = pc;
            self.accumulated = accumulation;

            let curr_ins = self.instruction_stack.pop().unwrap();
            self.processed.remove(&prev_pc);
            match curr_ins {
                Instruction::Accumulate(_) => continue,
                _ => {
                    if let Some(_) = self.tried_converting.get(&prev_pc) {
                        continue;
                    }
                }
            }
            self.tried_converting.insert(prev_pc);
            return Some(prev_pc);
        }

        None
    }
}

#[aoc_generator(day8)]
pub fn generate_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

/// ```
/// use advent_of_code_2020::day8::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day8.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 1384);
/// ```
#[aoc(day8, part1)]
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

/// ```
/// use advent_of_code_2020::day8::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day8.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 761);
/// ```
#[aoc(day8, part2, buggy)]
pub fn solve_part2(instructions: &Vec<Instruction>) -> i32 {
    let length: i32 = instructions.len() as i32;
    let mut i: u32 = 0;
    let mut console = GameConsole::new();
    let mut should_change_next = false;

    while i < length as u32 {
        let instruction = &instructions[i as usize];
        let next_instruction = if should_change_next {
            should_change_next = false;
            match instruction {
                Instruction::Accumulate(v) => Instruction::Accumulate(*v),
                Instruction::Jump(v) => Instruction::Noop(*v),
                Instruction::Noop(v) => Instruction::Jump(*v),
            }
        } else {
            *instruction
        };

        if let Some(next) = console.execute(i as u32, &next_instruction) {
            i = next;
        } else if let Some(next) = console.revert_to_last_breakpoint() {
            i = next;
            should_change_next = true;
        } else if i != (length - 1) as u32 {
            i = console.revert_to_last_breakpoint().unwrap();
            should_change_next = true;
        } else {
            break;
        }
    }
    console.accumulated
}

/// ```
/// use advent_of_code_2020::day8::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day8.txt").unwrap();
/// assert_eq!(solve_part2_without_stack(&generate_input(&input)), 761);
/// ```
#[aoc(day8, part2, without_stack)]
pub fn solve_part2_without_stack(instructions: &Vec<Instruction>) -> i32 {
    let length: i32 = instructions.len() as i32;

    'outer: for n in 0..length {
        let swapped: Vec<Instruction> = instructions
            .iter()
            .enumerate()
            .map(|(j, &instruction)| {
                if j as i32 == n {
                    match &instruction {
                        Instruction::Accumulate(v) => Instruction::Accumulate(*v),
                        Instruction::Noop(v) => Instruction::Jump(*v),
                        Instruction::Jump(v) => Instruction::Noop(*v),
                    }
                } else {
                    instruction
                }
            })
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
}
