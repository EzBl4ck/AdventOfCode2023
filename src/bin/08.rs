advent_of_code::solution!(8);
use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug)]
struct Pair<'a> {
    left: &'a str,
    right: &'a str,
}

impl Pair<'_> {
    fn new<'a>(left: &'a str, right: &'a str) -> Pair<'a> {
        Pair { left, right }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut map: HashMap<&str, Pair> = HashMap::new();

    let mut lines = input.lines();
    let moves = lines.next().unwrap().to_string();
    lines.next();

    for line in lines {
        let (key, values) = line.split_once(" = ").unwrap();
        let (v1, v2) = values[1..=values.len() - 2].split_once(", ").unwrap();
        map.insert(key, Pair::new(v1, v2));
    }

    let mut mi = 0;
    let mut steps = 1;
    let mut next = "AAA";

    loop {
        let direction = moves.chars().nth(mi).unwrap();
        let pair = map.get(next).unwrap();

        next = match direction {
            'R' => pair.right,
            'L' => pair.left,
            _ => pair.right,
        };

        match next == "ZZZ" {
            true => break,
            false => {
                steps += 1;
                mi = (mi + 1) % moves.len();
            }
        }
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map: HashMap<&str, Pair> = HashMap::new();
    let mut start: Vec<&str> = Vec::new();

    let mut lines = input.lines();
    let moves = lines.next().unwrap().to_string();
    lines.next();

    for line in lines {
        let (key, values) = line.split_once(" = ").unwrap();
        let (v1, v2) = values[1..=values.len() - 2].split_once(", ").unwrap();

        if key.chars().last().unwrap() == 'A' {
            start.push(key)
        };

        map.insert(key, Pair::new(v1, v2));
    }
    let mut total: Vec<usize> = Vec::new();
    for s in start {
        let mut mi = 0;
        let mut steps = 1;
        let mut next = s;

        loop {
            let direction = moves.chars().nth(mi).unwrap();
            let pair = map.get(next).unwrap();

            next = match direction {
                'R' => pair.right,
                'L' => pair.left,
                _ => pair.right,
            };

            match next.chars().last().unwrap() {
                'Z' => break,
                _ => {
                    steps += 1;
                    mi = (mi + 1) % moves.len();
                }
            }
        }
        total.push(steps);
    }

    let total = total.iter().fold(1, |acc, &x| lcm(acc, x));
    // Bruteforce solution that takes forever:
    // while !contains_all_zs(&start) {
    //  let direction = moves[mi] where mi is cycled until we exit from the loop
    //  start.map(|element| {
    //      // transform every element to the next element following the instruction and increment
    //      the counter
    //  }).collect()
    // }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
