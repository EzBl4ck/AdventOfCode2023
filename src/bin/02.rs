advent_of_code::solution!(2);

#[derive(Debug)]
struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

impl Turn {
    fn is_valid(&self) -> bool {
        if self.red > 12 || self.green > 13 || self.blue > 14 {
            false
        } else {
            true
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    'lines: for line in input.lines() {
        let mut parts = line.split(":");

        let id: u32 = parts.next()?.split_whitespace().nth(1)?.parse().ok()?;
        let turns: &str = parts.next()?.trim();

        for turn in turns.split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for color in turn.split(",") {
                let mut pair = color.trim().split_whitespace();
                let num: u32 = pair.next()?.parse().ok()?;

                match pair.next()? {
                    "red" => red = num,
                    "green" => green = num,
                    "blue" => blue = num,
                    _ => (),
                }
            }

            let t = Turn { red, green, blue };
            if !t.is_valid() {
                continue 'lines;
            }
        }

        sum += id;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let turns = line.split(":").nth(1)?.trim();

        for turn in turns.split(";") {
            for color in turn.split(",") {
                let mut pair = color.trim().split_whitespace();
                let num: u32 = pair.next()?.parse().ok()?;

                match pair.next()? {
                    "red" => {
                        if num > red {
                            red = num;
                        }
                    }
                    "green" => {
                        if num > green {
                            green = num;
                        }
                    }
                    "blue" => {
                        if num > blue {
                            blue = num;
                        }
                    }
                    _ => (),
                }
            }
        }

        let power = red * green * blue;
        sum += power;
    }

    Some(sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
