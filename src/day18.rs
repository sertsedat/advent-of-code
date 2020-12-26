fn is_operator(s: &str) -> bool {
    s == "*" || s == "+"
}

fn is_operand(s: &str) -> bool {
    s.parse::<u64>().is_ok()
}

fn precedence_part1(_s: &str, _t: &str) -> bool {
    true
}

fn precedence_part2(s: &str, t: &str) -> bool {
    match (s, t) {
        ("+", "*") => true,
        ("*", "*") => true,
        ("+", "+") => true,
        _ => false,
    }
}

fn infix_to_postfix<F>(infix: &Vec<String>, precedence: F) -> Vec<String>
where
    F: Fn(&str, &str) -> bool,
{
    let mut infix: Vec<&str> = infix.iter().map(|l| l.as_ref()).collect();
    infix.push(")");

    let mut postfix: Vec<&str> = Vec::new();
    let mut stack = vec!["("];

    for item in infix {
        match item {
            "(" => stack.push(item),
            v if is_operand(v) => {
                postfix.push(item);
            }
            o if is_operator(o) => {
                let x = stack.pop();
                if x.is_some() {
                    let mut y = x.unwrap();
                    while is_operator(y) && precedence(y, item) {
                        postfix.push(y);
                        y = stack.pop().unwrap();
                    }
                    stack.push(y);
                }

                stack.push(item);
            }
            ")" => {
                while let Some(x) = stack.pop() {
                    if x == "(" {
                        break;
                    }
                    postfix.push(x);
                }
            }
            _ => panic!("invalid infix"),
        }
    }

    postfix.iter().map(|a| a.to_string()).collect()
}

fn evaluate_postfix(postfix: &Vec<String>) -> u64 {
    let mut stack: Vec<u64> = Vec::new();

    for elem in postfix {
        if is_operand(elem) {
            stack.push(elem.parse().unwrap());
        } else {
            let value1 = stack.pop().unwrap();
            let value2 = stack.pop().unwrap();
            match elem.as_ref() {
                "*" => stack.push(value1 * value2),
                "+" => stack.push(value1 + value2),
                _ => unreachable!(),
            }
        }
    }

    stack.pop().unwrap()
}

#[aoc_generator(day18, part1)]
pub fn generate_input_part1(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l: &str| {
            let l = l.replace("(", " ( ").replace(")", " ) ").replace("  ", " ");
            let tokens = l.split_whitespace().map(|l| l.to_string()).collect();
            infix_to_postfix(&tokens, precedence_part1)
        })
        .collect()
}

/// ```
/// use advent_of_code_2020::day18::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day18.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input_part1(&input)), 6640667297513);
/// ```
#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<Vec<String>>) -> u64 {
    input.iter().map(|postfix| evaluate_postfix(postfix)).sum()
}

#[aoc_generator(day18, part2)]
pub fn generate_input_part2(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l: &str| {
            let l = l.replace("(", " ( ").replace(")", " ) ").replace("  ", " ");
            let tokens = l.split_whitespace().map(|l| l.to_string()).collect();
            infix_to_postfix(&tokens, precedence_part2)
        })
        .collect()
}

/// ```
/// use advent_of_code_2020::day18::*;
/// use std::fs;
/// let input = fs::read_to_string("input/2020/day18.txt").unwrap();
/// assert_eq!(solve_part1(&generate_input_part1(&input)), 6640667297513);
/// ```
#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<Vec<String>>) -> u64 {
    input.iter().map(|postfix| evaluate_postfix(postfix)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let text = "1 + 2 * 3 + 4 * 5 + 6
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
        let input = generate_input_part1(text);
        let actual = solve_part1(&input);

        assert_eq!(26406, actual);
    }

    #[test]
    fn example_part2() {
        let text = "1 + 2 * 3 + 4 * 5 + 6
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";
        let input = generate_input_part2(text);
        let actual = solve_part2(&input);

        assert_eq!(694122, actual);
    }
}
