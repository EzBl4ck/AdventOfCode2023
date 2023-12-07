advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    matches: usize,
}

impl Card {
    fn build(data: &str) -> Card {
        let (to_match, my_numbers) = data.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let to_match: Vec<usize> = to_match
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let my_numbers: Vec<usize> = my_numbers
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let matches = to_match.iter().filter(|n| my_numbers.contains(n)).count();

        Card { matches }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input.lines().map(Card::build).collect();
    let points: usize = cards
        .iter()
        .map(|card| {
            if card.matches == 0 {
                return 0;
            }
            usize::pow(2, (card.matches - 1) as u32)
        })
        .sum();
    Some(points)
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards: Vec<Card> = input.lines().map(Card::build).collect();
    let mut counts = vec![1; cards.len()];
    let len = counts.len() - 1;

    for (i, card) in cards.iter().enumerate() {
        let j = if card.matches > len {
            len
        } else {
            card.matches + i
        };

        for k in i + 1..j + 1 {
            counts[k] += counts[i];
        }
    }

    Some(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }
}
