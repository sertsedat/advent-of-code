use std::collections::HashSet;

// --- Day 1: Report Repair ---

#[aoc_generator(day1)]
pub fn generate_input(input: &str) -> HashSet<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

/*
 * Part 1
 *
 * After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island.
 * Surely, Christmas will go on without you.
 *
 * The tropical island has its own currency and is entirely cash-only.
 * The gold coins used there have a little picture of a starfish; the locals just call them stars.
 *  None of the currency exchanges seem to have heard of them,
 * but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
 *
 * To save your vacation, you need to get all fifty stars by December 25th.
 *
 *
 * Collect stars by solving puzzles.
 *
 * Two puzzles will be made available on each day in the Advent calendar;
 * the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
 *
 * Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input);
 * apparently, something isn't quite adding up.
 *
 * Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
 *
 * For example, suppose your expense report contained the following:
 *
 * 1721
 * 979
 * 366
 * 299
 * 675
 * 1456
 *
 * In this list, the two entries that sum to 2020 are 1721 and 299.
 * Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
 *
 * Of course, your expense report is much larger.
 * Find the two entries that sum to 2020; what do you get if you multiply them together?
 */
#[aoc(day1, part1)]
pub fn solve_part1(input: &HashSet<u32>) -> u32 {
    let target = 2020;

    for x in input {
        let y = target - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

/*
 * Part Two
 *
 * The Elves in accounting are thankful for your help;
 * one of them even offers you a starfish coin they had left over from a past vacation.
 * They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
 *
 * Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
 * Multiplying them together produces the answer, 241861950.
 *
 * In your expense report, what is the product of the three entries that sum to 2020?
*/
#[aoc(day1, part2)]
pub fn solve_part2(input: &HashSet<u32>) -> u32 {
    let target = 2020;

    for x in input {
        for y in input {
            if x + y > target {
                continue;
            }

            let z = target - x - y;

            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> HashSet<u32> {
        let text = "1721
979
366
299
675
1456
";
        generate_input(text)
    }

    #[test]
    fn example_part1() {
        let input = get_input();

        assert_eq!(solve_part1(&input), 514579);
    }

    #[test]
    fn example_part2() {
        let input = get_input();

        assert_eq!(solve_part2(&input), 241861950);
    }
}
