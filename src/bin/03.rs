advent_of_code::solution!(3);

use std::collections::HashSet;
use std::usize;

#[derive(PartialEq, Eq, Hash)]
struct Part {
    line_idx: usize,
    col_start: usize,
    col_end: usize,
    value: usize,
}

impl Part {
    fn from_position(lines: &[Vec<char>], i: usize, j: usize) -> Option<Part> {
        if !lines[i][j].is_ascii_digit() {
            return None;
        }

        let mut start = j;
        while start > 0 && lines[i][start - 1].is_ascii_digit() {
            start -= 1;
        }

        let mut end = j;
        while end < lines[i].len() - 1 && lines[i][end + 1].is_ascii_digit() {
            end += 1;
        }

        let number: String = lines[i][start..end + 1].iter().collect();
        let number = number.parse().ok()?;
        Some(Part {
            line_idx: i,
            col_start: start,
            col_end: end,
            value: number,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut parts = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if !is_symbol(*char) {
                continue;
            }

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let adjacent: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| in_bounds(&lines, *i, *j))
                .flat_map(|(i, j)| Part::from_position(&lines, *i, *j))
                .collect();

            parts.extend(adjacent);
        }
    }

    let p1: usize = parts.iter().map(|p| p.value).sum();
    Some(p1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut gear_ratios = vec![];
    let mut parts = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if !is_symbol(*char) {
                continue;
            }

            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let adjacent: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| in_bounds(&lines, *i, *j))
                .flat_map(|(i, j)| Part::from_position(&lines, *i, *j))
                .collect();

            if is_gear(*char, adjacent.len()) {
                let gear_ratio = adjacent.iter().map(|p: &Part| p.value).product();
                gear_ratios.push(gear_ratio);
            }

            parts.extend(adjacent);
        }
    }

    let p2: usize = gear_ratios.iter().sum();
    Some(p2)
}

fn in_bounds(lines: &[Vec<char>], i: usize, j: usize) -> bool {
    lines.get(i).and_then(|l| l.get(j)).is_some()
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn is_gear(c: char, adjacent: usize) -> bool {
    c == '*' && adjacent == 2
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(467835));
    }
}
