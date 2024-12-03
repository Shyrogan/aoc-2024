use regex::Regex;

advent_of_code::solution!(3);

type Mul = (u32, u32);
const REGEX_PATTERN: &str = r"mul\((\d+),\s*(\d+)\)";

fn captures_mul(line: &str) -> Vec<Mul> {
    let regex = Regex::new(REGEX_PATTERN).unwrap();
    regex.captures_iter(line)
        .map(|c| c.extract())
        .map(|(_, [a, b])| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(captures_mul)
        .map(|v| v.into_iter().map(|(a, b)| a * b).sum::<u32>())
        .sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
