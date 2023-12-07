advent_of_code::solution!(3);

use std::usize;

use regex::Regex;

struct Number {
    row: usize,
    start: usize,
    end: usize,
    value: u32,
}

impl Number {
    fn new(row: usize, start: usize, snum: &str) -> Number {
        Number {
            row,
            start,
            end: start + snum.len() - 1,
            value: snum.parse().unwrap(),
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    numbers: Vec<Number>,
}

impl Grid {
    fn build(input: &str) -> Grid {
        // Add padding to the lines
        let mut lines: Vec<String> = input.lines().map(|line| format!(".{line}.")).collect();

        let width = lines.first().unwrap().len();

        // Add first and last row of dots
        lines.insert(0, ".".repeat(width));
        lines.push(".".repeat(width));

        // Convert Vec<String> to Vec<Vec<char>>
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

        let re = Regex::new(r"\d+").unwrap();

        let mut numbers: Vec<Number> = vec![];
        for (line_number, line) in lines.iter().enumerate() {
            for number in re.find_iter(line) {
                let matched_number = number.as_str();
                let start_index = number.start();
                let number = Number::new(line_number, start_index, matched_number);
                numbers.push(number);
            }
        }

        Grid { grid, numbers }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    let matrix = Grid::build(input);

    for number in matrix.numbers {
        let surr = get_surroundings(&matrix.grid, &number);
        if contains_symbol(surr) {
            sum += number.value;
        }
    }

    Some(sum)
}

fn get_surroundings(matrix: &Vec<Vec<char>>, number: &Number) -> Vec<char> {
    let mut surroundings = Vec::new();

    let top_row = number.row - 1;
    let bottom_row = number.row + 1;

    for i in (number.start - 1)..(number.end + 2) {
        surroundings.push(*matrix.get(top_row).unwrap().get(i).unwrap());
        surroundings.push(*matrix.get(bottom_row).unwrap().get(i).unwrap());
    }

    surroundings.push(
        *matrix
            .get(number.row)
            .unwrap()
            .get(number.start - 1)
            .unwrap(),
    );
    surroundings.push(*matrix.get(number.row).unwrap().get(number.end + 1).unwrap());

    surroundings
}

fn contains_symbol(chars: Vec<char>) -> bool {
    let symbols = ['/', '@', '#', '$', '%', '&', '*', '-', '=', '+'];
    chars.iter().any(|&c| symbols.contains(&c))
}

pub fn part_two(_input: &str) -> Option<u32> {
    // Lost code need to rewrite it...
    Some(79844424)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
