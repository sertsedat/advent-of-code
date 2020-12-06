use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BYR_REGEX: Regex = Regex::new(r"byr:(\d+)").unwrap();
    static ref IYR_REGEX: Regex = Regex::new(r"iyr:(\d+)").unwrap();
    static ref EYR_REGEX: Regex = Regex::new(r"eyr:(\d+)").unwrap();
    static ref HGT_REGEX: Regex = Regex::new(r"hgt:(\w+)").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"hcl:(#?\w+)").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"ecl:(\w+)").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"pid:(\w+)").unwrap();
    static ref CID_REGEX: Regex = Regex::new(r"cid:(\w+)").unwrap();
    static ref HAIR_COLOR_VALIDATOR_REGEX: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref EYE_COLOR_VALIDATOR_REGEX: Regex =
        Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
}

#[derive(Debug)]
pub struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn to_string_value(capture: Option<regex::Captures>) -> Option<String> {
    match capture {
        Some(value) => Some(value.get(1)?.as_str().to_string()),
        None => None,
    }
}

fn to_int_value(capture: Option<regex::Captures>) -> Option<u32> {
    match capture {
        Some(value) => value.get(1)?.as_str().parse().ok(),
        None => None,
    }
}

impl Passport {
    pub fn new(line: &str) -> Option<Passport> {
        let byr = BYR_REGEX.captures(&line);
        let iyr = IYR_REGEX.captures(&line);
        let eyr = EYR_REGEX.captures(&line);
        let hgt = HGT_REGEX.captures(&line);
        let hcl = HCL_REGEX.captures(&line);
        let ecl = ECL_REGEX.captures(&line);
        let pid = PID_REGEX.captures(&line);
        let cid = CID_REGEX.captures(&line);

        let passport = Passport {
            byr: to_int_value(byr),
            iyr: to_int_value(iyr),
            eyr: to_int_value(eyr),
            hgt: to_string_value(hgt),
            hcl: to_string_value(hcl),
            ecl: to_string_value(ecl),
            pid: to_string_value(pid),
            cid: to_string_value(cid),
        };

        return if passport.has_all_required_fields() {
            Some(passport)
        } else {
            None
        };
    }

    pub fn has_all_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        (1920..=2002).contains(&self.byr.unwrap())
            && (2010..=2020).contains(&self.iyr.unwrap())
            && (2020..=2030).contains(&self.eyr.unwrap())
            && self.is_height_valid()
            && self.pid.to_owned().unwrap().len() == 9
            && HAIR_COLOR_VALIDATOR_REGEX
                .captures(&self.hcl.to_owned().unwrap())
                .is_some()
            && EYE_COLOR_VALIDATOR_REGEX
                .captures(&self.ecl.to_owned().unwrap())
                .is_some()
    }

    fn is_height_valid(&self) -> bool {
        let height = self.hgt.to_owned().unwrap();

        if height.ends_with("in") {
            let height = height.replace("in", "");
            let height: u32 = height.parse().unwrap();
            return (59..=76).contains(&height);
        } else if height.ends_with("cm") {
            let height = height.replace("cm", "");
            let height: u32 = height.parse().unwrap();
            return (150..=193).contains(&height);
        }
        false
    }
}

#[aoc_generator(day4)]
pub fn generate_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .filter_map(|line| {
            let line = line.replace("\n", " ");
            let passport = Passport::new(&line);
            passport
        })
        .collect()
}

/// ```
/// use advent_of_code_2020::day4::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day4.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input(&input)), 196);
/// ```
#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Passport>) -> usize {
    input.len()
}

/// ```
/// use advent_of_code_2020::day4::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day4.txt").unwrap();
/// assert_eq!(solve_part2(&generate_input(&input)), 167);
/// ```
#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Passport>) -> usize {
    input.iter().filter(|passport| passport.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let text = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023
pid:028048884 hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013 eyr:2024
ecl:brn pid:760753108 byr:1931 hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let input = generate_input(text);
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    fn example_part2_invalid() {
        let text = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let input = generate_input(text);
        assert_eq!(solve_part2(&input), 0);
    }

    #[test]
    fn example_part2_valid() {
        let text = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let input = generate_input(text);
        assert_eq!(solve_part2(&input), 4);
    }
}
