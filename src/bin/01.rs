use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();
    for line in input.split("\n").filter(|s| !s.is_empty()) {
        let (ns1, ns2) = line.split_once("   ").expect("Line has no space ?");
        let (n1, n2) = (ns1.parse::<u32>().expect("Expected a number as first element"), ns2.parse::<u32>().expect("Expected a number as second element"));
        l1.push(n1);
        l2.push(n2);
    }
    l1.sort();
    l2.sort();

    let sum: u32 = l1.iter()
        .zip(l2.iter())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: HashMap<u32, u32> = HashMap::new();
    for line in input.split("\n").filter(|s| !s.is_empty()) {
        let (ns1, ns2) = line.split_once("   ").expect("Line has no space ?");
        let (n1, n2) = (ns1.parse::<u32>().expect("Expected a number as first element"), ns2.parse::<u32>().expect("Expected a number as second element"));
        l1.push(n1);
        l2.insert(n2, l2.get(&n2).unwrap_or(&0) + 1);
    }
    let sum: u32 = l1.iter()
        .map(|x| x * l2.get(x).unwrap_or(&0))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
