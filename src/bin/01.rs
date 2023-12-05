advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut tot = 0;

    for line in input.lines() {
        let mut num1 = 0;
        let mut num2 = 0;

        for char in line.chars() {
            if char.is_numeric() {
                num1 = char.to_digit(10).unwrap();
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                num2 = char.to_digit(10).unwrap();
                break;
            }
        }

        let num = num1 * 10 + num2;
        tot += num;
    }
    Some(tot)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)|([1-9])").unwrap();

    for line in input.lines() {
        let first = match re.find(line) {
            Some(first) => first.as_str(),
            _ => continue,
        };

        let end = line.len();
        let mut start = line.len() - 1;

        let second = loop {
            let sub = &line[start..end];
            if let Some(second) = re.find(sub) {
                break second.as_str();
            } else {
                start -= 1;
                continue;
            }
        };

        let num1 = get_number(&first);
        let num2 = get_number(&second);
        let num = num1 * 10 + num2;
        sum += num;
    }

    Some(sum)
}

fn get_number(string: &str) -> u32 {
    match string {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
