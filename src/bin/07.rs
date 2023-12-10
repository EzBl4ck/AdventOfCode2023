advent_of_code::solution!(7);
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    label: HandType,
    cards: String,
    bid: usize,
    wildcard: bool,
}

impl Hand {
    fn build(cards: String, bid: usize, wildcard: bool) -> Hand {
        let label = match wildcard {
            true => classify_max_hand(&cards),
            false => classify_hand(&cards),
        };

        Hand {
            cards,
            bid,
            label,
            wildcard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let label_order = self.label.cmp(&other.label);

        match label_order {
            Ordering::Equal => {
                let strengths;

                match self.wildcard {
                    true => {
                        strengths = vec![
                            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
                        ]
                    }
                    false => {
                        strengths = vec![
                            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
                        ]
                    }
                }

                let self_chars = self.cards.chars();
                let other_chars = other.cards.chars();

                for (sc, oc) in self_chars.zip(other_chars) {
                    let sc = strengths.iter().position(|&c| c == sc).unwrap();
                    let oc = strengths.iter().position(|&c| c == oc).unwrap();

                    if sc == oc {
                        continue;
                    }

                    return sc.cmp(&oc);
                }
                return Ordering::Equal;
            }
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn classify_hand(cards: &str) -> HandType {
    let mut counter: Vec<usize> = vec![0; 13];

    cards.chars().for_each(|c| {
        //println!("{c}");
        let idx = match c {
            'A' => 0,
            'K' => 1,
            'Q' => 2,
            'J' => 3,
            'T' => 4,
            c => {
                let digit = c.to_digit(10).unwrap();
                digit + 3
            }
        };
        counter[idx as usize] += 1;
    });

    counter.retain(|&x| x != 0);

    match counter.len() {
        1 => HandType::FiveOfAKind,
        2 if *counter.iter().max().unwrap() == 4 => HandType::FourOfAKind,
        2 => HandType::FullHouse,
        3 if *counter.iter().max().unwrap() == 3 => HandType::ThreeOfAKind,
        3 => HandType::TwoPair,
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn classify_max_hand(cards: &str) -> HandType {
    if !cards.contains('J') {
        return classify_hand(cards);
    }

    let mut max_type = HandType::HighCard;
    let mut cards = cards.chars().collect::<Vec<char>>();

    max_type = find_max(&mut cards, 0, max_type);

    return max_type;
}

fn find_max(cards: &mut Vec<char>, index: usize, current_max: HandType) -> HandType {
    if index == cards.len() {
        return classify_hand(cards.clone().into_iter().collect::<String>().as_str());
    }

    if cards[index] != 'J' {
        return find_max(cards, index + 1, current_max);
    }

    let possible_cards: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    let mut max_type = current_max;

    for c in possible_cards {
        cards[index] = c;
        max_type = std::cmp::max(max_type, find_max(cards, index + 1, max_type));
    }
    cards[index] = 'J';

    max_type
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let pair = line.split_once(" ").unwrap();
            let cards = pair.0.to_string();
            let bid = pair.1.parse().unwrap();
            Hand::build(cards, bid, false)
        })
        .collect();

    hands.sort_by(|a, b| a.cmp(b));

    //for hand in &hands {
    //    println!("{:?}", hand);
    //}

    let mut total = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let rank = idx + 1;
        total += hand.bid * rank;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let pair = line.split_once(" ").unwrap();
            let cards = pair.0.to_string();
            let bid = pair.1.parse().unwrap();
            Hand::build(cards, bid, true)
        })
        .collect();

    hands.sort_by(|a, b| a.cmp(b));

    //for hand in &hands {
    //    println!("{:?}", hand);
    //}

    let mut total = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let rank = idx + 1;
        total += hand.bid * rank;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
