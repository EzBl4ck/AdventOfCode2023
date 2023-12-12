advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<isize> {
    let mut total = 0;
    for line in input.lines() {
        let mut values: Vec<Vec<isize>> = Vec::new();
        values.push(
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );

        while !values.last().unwrap().iter().all(|n| *n == 0) {
            values.push({
                let last_values = values.last().unwrap();
                let mut new_values = Vec::new();

                for i in 1..last_values.len() {
                    new_values.push(last_values[i] - last_values[i - 1]);
                }

                new_values
            })
        }

        let mut partial = 0;
        for i in (0..values.len()).rev() {
            values[i].push(partial);

            match i {
                0 => total += partial,
                _ => partial += values[i - 1].last().unwrap(),
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut total = 0;
    for line in input.lines() {
        let mut values: Vec<Vec<isize>> = Vec::new();
        values.push(
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );

        while !values.last().unwrap().iter().all(|n| *n == 0) {
            values.push({
                let last_values = values.last().unwrap();
                let mut new_values = Vec::new();

                for i in 1..last_values.len() {
                    new_values.push(last_values[i] - last_values[i - 1]);
                }

                new_values
            })
        }

        let mut partial = 0;
        for i in (0..values.len()).rev() {
            values[i].insert(0, partial);

            match i {
                0 => total += partial,
                _ => partial = values[i - 1].first().unwrap() - partial,
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
