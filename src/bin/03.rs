use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
enum Operation {
    Do,
    Dont,
    Mul(u32, u32)
}

const REGEX_PATTERN: &str = r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)";

fn captures_op(line: &str) -> Vec<Operation> {
    let regex = Regex::new(REGEX_PATTERN).unwrap();
    regex.captures_iter(line.to_lowercase().as_str())
        .filter_map(|caps| {
            // Check which part of the regex matched
            match caps.get(1) {
                Some(a) => Some(Operation::Mul(a.as_str().parse::<u32>().unwrap(), caps.get(2).unwrap().as_str().parse::<u32>().unwrap())),
                None => {
                    match caps.get(0).unwrap().as_str() {
                        "do()" => Some(Operation::Do),
                        "don't()" => Some(Operation::Dont),
                        _ => None,
                    }
                }
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(captures_op)
        .map(|v| v.into_iter()
            .map(|op| match op {
                Operation::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum::<u32>())
        .sum::<u32>())

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    Some(input.split_whitespace()
        .filter(|s| !s.is_empty())
        .map(captures_op)
        .map(|v| v.into_iter()
            .map(|op| match op {
                Operation::Do => { enabled = true; 0 },
                Operation::Dont => { enabled = false; 0 },
                Operation::Mul(_, _) if !enabled => 0,
                Operation::Mul(a, b) => a * b,
            })
            .sum::<u32>())
        .sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do() {
        let result = captures_op("do()");
        assert_eq!(result, vec![Operation::Do]);
    }

    #[test]
    fn test_dont() {
        let result = captures_op("^don't()_mul(5,5");
        assert_eq!(result, vec![Operation::Dont]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
