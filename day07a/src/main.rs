use std::{cmp::Ordering, fs};

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(line: &str) -> Hand {
        let (card_str, bid_str) = line.split_once(" ").unwrap();
        let mut cards = Vec::new();
        for c in card_str.chars() {
            if let Some(digit) = c.to_digit(10) {
                cards.push(digit);
            } else if c == 'T' {
                cards.push(10);
            } else if c == 'J' {
                cards.push(11);
            } else if c == 'Q' {
                cards.push(12);
            } else if c == 'K' {
                cards.push(13);
            } else if c == 'A' {
                cards.push(14);
            }
        }

        // Determine the type of hand
        let mut dedup_cards = cards.clone();
        dedup_cards.sort();
        dedup_cards.dedup();
        let highest_dup_count = dedup_cards
            .iter()
            .map(|dedup_card| cards.iter().filter(|&card| card == dedup_card).count())
            .max();
        let hand_type: HandType = if dedup_cards.len() == 1 {
            HandType::FiveOfAKind
        } else if dedup_cards.len() == 2 && highest_dup_count == Some(4) {
            HandType::FourOfAKind
        } else if dedup_cards.len() == 2 && highest_dup_count == Some(3) {
            HandType::FullHouse
        } else if dedup_cards.len() == 3 && highest_dup_count == Some(3) {
            HandType::ThreeOfAKind
        } else if dedup_cards.len() == 3 && highest_dup_count == Some(2) {
            HandType::TwoPair
        } else if dedup_cards.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Hand {
            cards,
            bid: bid_str.parse::<u64>().unwrap(),
            hand_type,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            Some(Ordering::Greater)
        } else if self.hand_type < other.hand_type {
            Some(Ordering::Less)
        } else {
            let ordering: Ordering = self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(lhs, rhs)| {
                    if lhs > rhs {
                        Some(Ordering::Greater)
                    } else if lhs < rhs {
                        Some(Ordering::Less)
                    } else {
                        None
                    }
                })
                .flatten()
                .next()
                .unwrap_or(Ordering::Equal);
            Some(ordering)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Hand {}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");

    let mut hands = data
        .lines()
        .map(|line| Hand::new(line))
        .collect::<Vec<Hand>>();
    hands.sort();

    for (rank_minus_one, hand) in hands.iter().enumerate() {
        println!("rank {} -> {:?}", rank_minus_one + 1, hand);
    }

    let result: u64 = hands
        .iter()
        .enumerate()
        .map(|(rank_minus_one, hand)| ((rank_minus_one as u64) + 1) * hand.bid)
        .sum();

    println!("{}", result);
}
