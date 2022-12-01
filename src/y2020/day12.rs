use std::str::FromStr;

pub enum Instruction {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Right(i32),
    Left(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = s[1..].parse().unwrap();

        Ok(match &s[..1] {
            "N" => Instruction::North(amount),
            "S" => Instruction::South(amount),
            "W" => Instruction::West(amount),
            "E" => Instruction::East(amount),
            "R" => Instruction::Right(amount),
            "L" => Instruction::Left(amount),
            "F" => Instruction::Forward(amount),
            _ => panic!("input not valid: {}", s),
        })
    }
}

struct Ferry {
    x: i32,
    y: i32,
    angle: i32,
}

impl Ferry {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            angle: 0,
        }
    }

    fn forward(&mut self, d: &i32) {
        let radians = (self.angle as f32).to_radians();

        let d = *d as f32;

        self.x += (d * radians.cos()).round() as i32;
        self.y += (d * radians.sin()).round() as i32;
    }

    fn to_waypoint(&mut self, waypoint: &Waypoint, amount: &i32) {
        self.x += waypoint.x * amount;
        self.y += waypoint.y * amount;
    }

    fn manhattan_distance(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn act(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(amount) => self.y += amount,
            Instruction::South(amount) => self.y -= amount,
            Instruction::East(amount) => self.x += amount,
            Instruction::West(amount) => self.x -= amount,
            Instruction::Left(amount) => self.angle += amount,
            Instruction::Right(amount) => self.angle -= amount,
            Instruction::Forward(amount) => self.forward(amount),
        }
    }
}

#[aoc_generator(day12)]
pub fn generate_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(instructions: &Vec<Instruction>) -> u32 {
    let mut ferry = Ferry::new();
    instructions.iter().for_each(|i| ferry.act(i));

    ferry.manhattan_distance()
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn turn(&mut self, angles: i32) {
        let radians = (angles as f32).to_radians();

        let x = self.x as f32;
        let y = self.y as f32;

        let sin = radians.sin();
        let cos = radians.cos();

        // 𝐶′𝑥=𝐵′𝑥cos𝛼−𝐵′𝑦sin𝛼 and 𝐶′𝑦=𝐵′𝑥sin𝛼+𝐵′𝑦cos𝛼
        let new_x = (x * cos - y * sin).round() as i32;
        let new_y = (x * sin + y * cos).round() as i32;

        self.x = new_x;
        self.y = new_y;
    }

    fn act(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(amount) => self.y += amount,
            Instruction::South(amount) => self.y -= amount,
            Instruction::East(amount) => self.x += amount,
            Instruction::West(amount) => self.x -= amount,
            Instruction::Left(amount) => self.turn(*amount),
            Instruction::Right(amount) => self.turn(-amount),
            _ => (),
        }
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(instructions: &Vec<Instruction>) -> u32 {
    let mut waypoint = Waypoint::new(10, 1);
    let mut ferry = Ferry::new();
    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Forward(amount) => ferry.to_waypoint(&waypoint, amount),
            _ => waypoint.act(instruction),
        });

    ferry.manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    fn get_input() -> Vec<Instruction> {
        let text = "F10
N3
F7
R90
F11
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        let actual = solve_part1(&input);
        assert_eq!(actual, 25);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        let actual = solve_part2(&input);
        assert_eq!(actual, 286);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day12.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(820, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day12.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(66614, actual);
    }
}
