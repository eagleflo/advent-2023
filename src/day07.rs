use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq)]
struct Card {
    rank: Rank,
}

impl Card {
    pub fn new(input: char) -> Self {
        let rank = match input {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => panic!("Unknown rank"),
        };
        Card { rank }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Win {
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    FiveOfAKind(Rank),
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    win: Win,
    bid: u32,
}

impl Hand {
    pub fn win(cards: &Vec<Card>) -> Win {
        let mut bins: HashMap<Rank, u8> = HashMap::new();
        for card in cards {
            if let Some(number) = bins.get_mut(&card.rank) {
                *number += 1;
            } else {
                bins.insert(card.rank, 1);
            }
        }

        let (rank, num) = bins.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        if *num == 5 {
            Win::FiveOfAKind(*rank)
        } else if *num == 4 {
            Win::FourOfAKind(*rank)
        } else if *num == 3 {
            if let Some((other_rank, _)) = bins.iter().find(|(_, n)| **n == 2) {
                Win::FullHouse(*rank, *other_rank)
            } else {
                Win::ThreeOfAKind(*rank)
            }
        } else if *num == 2 {
            let pairs: Vec<_> = bins.iter().filter(|(_, n)| **n == 2).collect();
            if pairs.len() == 2 {
                let ranks: Vec<_> = pairs.iter().map(|(r, _)| r).collect();
                Win::TwoPair(**ranks[0], **ranks[1])
            } else {
                Win::OnePair(*rank)
            }
        } else {
            let high = cards.iter().max().unwrap();
            Win::HighCard(high.rank)
        }
    }

    pub fn new(input: &str) -> Self {
        let (cards_str, bid_str) = input.split_once(' ').unwrap();
        let cards = cards_str.chars().map(Card::new).collect();
        let win = Hand::win(&cards);
        let bid = bid_str.parse::<u32>().unwrap();
        Hand { cards, win, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        dbg!(self, other);
        if mem::discriminant(&self.win) == mem::discriminant(&other.win) {
            self.cards.cmp(&other.cards)
        } else {
            self.win.cmp(&other.win)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.win == other.win
    }
}

pub fn solve_part_one() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
    let mut hands: Vec<Hand> = input.lines().map(Hand::new).collect();
    hands.sort();

    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index as u32 + 1) * hand.bid;
    }
    println!("06 - Part One: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_compares() {
        let c1 = Card::new('A');
        let c2 = Card::new('A');
        let c3 = Card::new('8');

        assert!(c1 == c2);
        assert!(c1 > c3);
    }

    #[test]
    fn test_hand_compares() {
        let hand1 = Hand::new("KK677 28");
        let hand2 = Hand::new("KTJJT 220");
        assert!(hand1 > hand2);

        let hand3 = Hand::new("T55J5 684");
        let hand4 = Hand::new("QQQJA 483");
        assert!(hand4 > hand3);
    }

    #[test]
    fn test_wins() {
        let five = Hand::new("22222 123");
        assert_eq!(five.win, Win::FiveOfAKind(Rank::Two));

        let full_house = Hand::new("22233 123");
        assert_eq!(full_house.win, Win::FullHouse(Rank::Two, Rank::Three));

        let two_pairs = Hand::new("33577 123");
        assert!(
            two_pairs.win == Win::TwoPair(Rank::Three, Rank::Seven)
                || two_pairs.win == Win::TwoPair(Rank::Seven, Rank::Three),
        );

        let one_pair = Hand::new("AA234 123");
        assert_eq!(one_pair.win, Win::OnePair(Rank::Ace));

        let high_card = Hand::new("A2345 123");
        assert_eq!(high_card.win, Win::HighCard(Rank::Ace));
    }
}
