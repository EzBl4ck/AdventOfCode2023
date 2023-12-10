advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn count_win(&self) -> usize {
        let mut win = 0;

        for i in 1..self.time {
            let running_time = self.time - i;
            if running_time * i > self.distance {
                win += 1;
            } else {
                if win > 0 {
                    break;
                }
            }
        }

        win
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let mut lines = input.lines();

    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    for (i, &time) in times.iter().enumerate() {
        races.push(Race {
            time,
            distance: distances[i],
        });
    }

    races
}

pub fn part_one(input: &str) -> Option<usize> {
    let races = parse_input(input);
    let mut total = 1;

    for race in races {
        total *= race.count_win();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let time: usize = lines
        .next()?
        .replace(" ", "")
        .split_once(":")?
        .1
        .parse()
        .unwrap();
    let distance: usize = lines
        .next()?
        .replace(" ", "")
        .split_once(":")?
        .1
        .parse()
        .unwrap();

    let race = Race { time, distance };

    Some(race.count_win())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
