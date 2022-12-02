use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Copy, Clone)]
pub enum RoundResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl Hand {
    /**
       Rock = 1 point
       Paper = 2 points
       Scissors = 3 points
    */
    fn value(self) -> u32 {
        self as u32 + 1
    }

    fn against(self, other: Hand) -> RoundResult {
        match self as i8 - other as i8 {
            0 => RoundResult::Draw,
            1 | -2 => RoundResult::Win,
            -1 | 2 => RoundResult::Loss,
            _ => panic!("unreachable"),
        }
    }
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("input is corrupt"),
        }
    }
}

impl TryFrom<i8> for Hand {
    type Error = ();

    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            x if x == Hand::Rock as i8 => Ok(Hand::Rock),
            x if x == Hand::Paper as i8 => Ok(Hand::Paper),
            x if x == Hand::Scissors as i8 => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

impl From<&str> for RoundResult {
    fn from(input: &str) -> Self {
        match input {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("input is corrupt"),
        }
    }
}

#[aoc_generator(day02, part1)]
pub fn generate_input_part1(input: &str) -> Vec<Vec<Hand>> {
    input
        .lines()
        // line consists of opponent hand, own hand
        .map(|line| line.split(" ").map(|c| Hand::from(c)).collect())
        .collect()
}

#[aoc_generator(day02, part2)]
pub fn generate_input_part2(input: &str) -> Vec<(Hand, RoundResult)> {
    input
        .lines()
        .map(|line| {
            // line consists of opponent hand, and result
            let v: Vec<&str> = line.split(" ").collect();
            (Hand::from(v[0]), RoundResult::from(v[1]))
        })
        .collect()
}

#[aoc(day02, part1)]
pub fn solve_part1(input: &Vec<Vec<Hand>>) -> u32 {
    input
        .iter()
        .map(|v| {
            let opponent = v[0];
            let me = v[1];
            me.against(opponent) as u32 + me.value()
        })
        .sum()
}

#[aoc(day02, part2)]
pub fn solve_part2(input: &Vec<(Hand, RoundResult)>) -> u32 {
    input
        .iter()
        .map(|(opponent, result)| {
            let me = match result {
                RoundResult::Draw => *opponent,
                // use previous possible hand, ie opponent is Paper, choose Rock
                RoundResult::Loss => Hand::try_from(((*opponent as i8) - 1 + 3) % 3).unwrap(),
                // use next possible hand, ie opponent is Rock, choose Paper
                RoundResult::Win => Hand::try_from(((*opponent as i8) + 1) % 3).unwrap(),
            };

            *result as u32 + me.value()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_input_part_1() -> Vec<Vec<Hand>> {
        let text = "A Y
B X
C Z";
        generate_input_part1(text)
    }

    fn get_input_part_2() -> Vec<(Hand, RoundResult)> {
        let text = "A Y
B X
C Z";
        generate_input_part2(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input_part_1();

        assert_eq!(15, solve_part1(&input));
    }

    #[test]
    fn example_part2() {
        let input = get_input_part_2();

        assert_eq!(12, solve_part2(&input));
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2022/day2.txt").unwrap();
        let input = generate_input_part1(&text);
        let actual = solve_part1(&input);
        assert_eq!(12772, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2022/day2.txt").unwrap();
        let input = generate_input_part2(&text);
        let actual = solve_part2(&input);
        assert_eq!(11618, actual);
    }
}
