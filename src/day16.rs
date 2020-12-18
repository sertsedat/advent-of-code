use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

lazy_static! {
    static ref TICKET_FIELD_REGEX: Regex = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

pub struct TicketFieldRule {
    name: String,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

impl TicketFieldRule {
    pub fn accepts_value(&self, val: u32) -> bool {
        self.range1.contains(&val) || self.range2.contains(&val)
    }
}

impl FromStr for TicketFieldRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match TICKET_FIELD_REGEX.captures(s) {
            Some(re) => {
                let name = re.get(1).unwrap().as_str().to_string();
                let range1_1: u32 = re.get(2).unwrap().as_str().parse().unwrap();
                let range1_2: u32 = re.get(3).unwrap().as_str().parse().unwrap();
                let range2_1: u32 = re.get(4).unwrap().as_str().parse().unwrap();
                let range2_2: u32 = re.get(5).unwrap().as_str().parse().unwrap();

                Ok(TicketFieldRule {
                    name,
                    range1: RangeInclusive::new(range1_1, range1_2),
                    range2: RangeInclusive::new(range2_1, range2_2),
                })
            }
            None => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Ticket(Vec<u32>);

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket(s.split(',').map(|x| x.parse().unwrap()).collect()))
    }
}

impl Ticket {
    fn sum_invalid_fields(&self, fields: &Vec<TicketFieldRule>) -> Option<u32> {
        let invalid: Vec<&u32> = self
            .0
            .iter()
            .filter(|&&v| fields.iter().all(|f| !f.accepts_value(v)))
            .collect();

        if invalid.is_empty() {
            None
        } else {
            Some(invalid.iter().copied().sum())
        }
    }
}

pub struct Train {
    ticket_field_rules: Vec<TicketFieldRule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[aoc_generator(day16)]
pub fn generate_input(input: &str) -> Train {
    let input: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|l| l.trim().split('\n').collect())
        .collect();

    let ticket_field_rules = input[0].iter().map(|l| l.parse().unwrap()).collect();

    let my_ticket: Ticket = input[1][1].parse().unwrap();

    let nearby_tickets: Vec<Ticket> = input[2][1..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    Train {
        ticket_field_rules,
        my_ticket,
        nearby_tickets,
    }
}

/// ```
/// use advent_of_code_2020::day16::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day16.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 23122);
/// ```
#[aoc(day16, part1)]
pub fn solve_part1(train: &Train) -> u32 {
    train
        .nearby_tickets
        .iter()
        .filter_map(|ticket| ticket.sum_invalid_fields(&train.ticket_field_rules))
        .sum()
}

/// ```
/// use advent_of_code_2020::day16::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day16.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 362974212989);
/// ```
#[aoc(day16, part2)]
pub fn solve_part2(train: &Train) -> u64 {
    let valid_nearby_tickets: Vec<&Ticket> = train
        .nearby_tickets
        .iter()
        .filter(|&ticket| ticket.sum_invalid_fields(&train.ticket_field_rules) == None)
        .collect();

    let mut departure_field_product = 1u64;
    let mut found_fields_indices = HashSet::new();
    let field_len = train.ticket_field_rules.len();

    while field_len != found_fields_indices.len() {
        'o: for i in 0..field_len {
            let mut found_index: isize = -1;
            for field_index in 0..field_len {
                if found_fields_indices.contains(&field_index) {
                    continue;
                }

                if valid_nearby_tickets
                    .iter()
                    .all(|ticket| train.ticket_field_rules[field_index].accepts_value(ticket.0[i]))
                {
                    if found_index != -1 {
                        continue 'o;
                    }
                    found_index = field_index as isize;
                }
            }
            if found_index == -1 {
                continue;
            }

            let ind = found_index as usize;
            found_fields_indices.insert(ind);
            if train.ticket_field_rules[ind].name.starts_with("departure") {
                let value = train.my_ticket.0[i] as u64;
                departure_field_product *= value;
            }
        }
    }

    departure_field_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let text = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let input = generate_input(text);
        let actual = solve_part1(&input);
        assert_eq!(actual, 71);
    }

    #[test]
    fn example_part2() {
        let text = "class: 0-1 or 4-19
row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let input = generate_input(text);
        let actual = solve_part2(&input);
        assert_eq!(actual, 13);
    }
}
