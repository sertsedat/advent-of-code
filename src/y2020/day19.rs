use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_RULE: Regex = Regex::new(r#""(\w)""#).unwrap();
}

#[derive(Debug, Clone)]
pub enum Rule {
    Char(char),
    SubRules(Vec<Vec<usize>>),
}

impl std::str::FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(capture) = CHAR_RULE.captures(s) {
            Ok(Rule::Char(
                capture.get(1).unwrap().as_str().chars().next().unwrap(),
            ))
        } else {
            let subrules: Vec<Vec<usize>> = s
                .split('|')
                .map(|r| r.split_whitespace().map(|t| t.parse().unwrap()).collect())
                .collect();

            Ok(Rule::SubRules(subrules))
        }
    }
}

fn parse_rule(s: &str) -> (usize, Rule) {
    let s: Vec<&str> = s.split(":").collect();

    (s[0].parse().unwrap(), s[1].parse().unwrap())
}

type Rules = HashMap<usize, Rule>;

#[aoc_generator(day19)]
pub fn generate_input(input: &str) -> (Rules, Vec<String>) {
    let mut lines = input.lines();

    let rules: Rules = (&mut lines)
        .take_while(|&line| !line.is_empty())
        .par_bridge()
        .map(parse_rule)
        .collect();

    let messages = lines.map(|l| l.to_string()).collect();

    (rules, messages)
}

fn matches<'a>(chars: &'a [char], index: &usize, rules: &Rules) -> Option<Vec<&'a [char]>> {
    if chars.is_empty() {
        return None;
    }

    match rules.get(index).unwrap() {
        Rule::Char(c) => {
            if chars[0] == *c {
                Some(vec![&chars[1..]])
            } else {
                None
            }
        }
        Rule::SubRules(subrules) => {
            let mut results = subrules
                .iter()
                .filter_map(|subrule| {
                    let mut subrule_results = vec![chars];
                    for rule_ind in subrule {
                        let mut next_subrule_results = subrule_results
                            .iter()
                            .filter_map(|r| matches(&r, &rule_ind, rules))
                            .peekable();
                        if next_subrule_results.peek().is_some() {
                            subrule_results = next_subrule_results.flatten().collect();
                        } else {
                            return None;
                        }
                    }
                    Some(subrule_results)
                })
                .peekable();
            if results.peek().is_some() {
                Some(results.flatten().collect())
            } else {
                None
            }
        }
    }
}

#[aoc(day19, part1)]
pub fn solve_part1((rules, messages): &(Rules, Vec<String>)) -> usize {
    messages
        .par_iter()
        .filter(|message| {
            let chars: Vec<_> = message.chars().collect();
            matches(&chars, &0, &rules)
                .map(|results| results.iter().any(|r| r.is_empty()))
                .unwrap_or(false)
        })
        .count()
}

#[aoc(day19, part2)]
pub fn solve_part2((rules, messages): &(Rules, Vec<String>)) -> usize {
    let mut rules = rules.clone();
    rules.insert(8, "42 | 42 8".parse().unwrap());
    rules.insert(11, "42 31 | 42 11 31".parse().unwrap());

    solve_part1(&(rules, messages.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example_part1() {
        let text = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb        
";
        let input = generate_input(text);
        let actual = solve_part1(&input);

        assert_eq!(2, actual);
    }

    #[test]
    fn example_part2() {
        let text = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";
        let input = generate_input(text);
        let actual = solve_part2(&input);

        assert_eq!(12, actual);
    }

    #[test]
    fn test_input_part1() {
        let text = fs::read_to_string("input/2020/day19.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part1(&input);
        assert_eq!(269, actual);
    }

    #[test]
    fn test_input_part2() {
        let text = fs::read_to_string("input/2020/day19.txt").unwrap();
        let input = generate_input(&text);
        let actual = solve_part2(&input);
        assert_eq!(403, actual);
    }
}
