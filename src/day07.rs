use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    Joker = 1,
    Two,
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

#[derive(Clone, Debug, Eq)]
struct Card {
    rank: Rank,
}

impl Card {
    pub fn new(input: char) -> Self {
        Self::new_with_joker(input, false)
    }

    pub fn new_with_joker(input: char, jokers: bool) -> Self {
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
        if jokers && rank == Rank::Jack {
            Card { rank: Rank::Joker }
        } else {
            Card { rank }
        }
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

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match &self.rank {
            Rank::Joker => 'J',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        };
        write!(f, "{}", rank)
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

#[derive(Eq)]
struct Hand {
    cards: Vec<Card>,
    win: Win,
    bid: u32,
}

impl Hand {
    pub fn win(cards: &Vec<Card>) -> Win {
        let mut bins: HashMap<Rank, u8> = HashMap::new();
        let mut number_of_jokers = 0;
        for card in cards {
            if card.rank == Rank::Joker {
                number_of_jokers += 1;
            }
            if let Some(number) = bins.get_mut(&card.rank) {
                *number += 1;
            } else {
                bins.insert(card.rank, 1);
            }
        }

        if number_of_jokers == 5 {
            return Win::FiveOfAKind(Rank::Joker);
        }

        // Add the jokers to the largest non-joker bin. In the case of a tie,
        // add to the highest ranking bin.
        let max = bins
            .iter()
            .filter(|(r, _)| **r != Rank::Joker)
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap();
        let ties: Vec<_> = bins
            .iter()
            .filter(|(r, n)| **r != Rank::Joker && **n == *max.1)
            .collect();
        let best_bin = ties.iter().max_by(|(x, _), (y, _)| x.cmp(y)).unwrap();
        let most = best_bin.0;
        if let Some(number) = bins.get_mut(&most.clone()) {
            *number += number_of_jokers;
        }

        let (rank, num) = bins.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        if *num == 5 {
            Win::FiveOfAKind(*rank)
        } else if *num == 4 {
            Win::FourOfAKind(*rank)
        } else if *num == 3 {
            if let Some((other_rank, _)) = bins
                .iter()
                .filter(|(r, _)| **r != Rank::Joker)
                .find(|(_, n)| **n == 2)
            {
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
        Self::new_with_joker(input, false)
    }

    pub fn new_with_joker(input: &str, joker: bool) -> Self {
        let (cards_str, bid_str) = input.split_once(' ').unwrap();
        let cards = cards_str
            .chars()
            .map(|c| Card::new_with_joker(c, joker))
            .collect();
        let win = Hand::win(&cards);
        let bid = bid_str.parse::<u32>().unwrap();
        Hand { cards, win, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
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

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cards = self.cards.iter().map(|c| c.to_string()).collect::<String>();
        write!(f, "{} - {:?} ({})", cards, self.win, self.bid)
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

pub fn solve_part_two() {
    let input = std::fs::read_to_string("input/07.txt").unwrap();
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| Hand::new_with_joker(l, true))
        .collect();
    hands.sort();

    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index as u32 + 1) * hand.bid;
    }
    println!("06 - Part Two: {}", total);
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

    #[test]
    fn test_joker_wins() {
        let hand1 = Hand::new_with_joker("JKKK2 123", true);
        let hand2 = Hand::new_with_joker("QQQQ2 123", true);
        assert!(hand1 < hand2);

        let hand3 = Hand::new_with_joker("T55J5 123", true);
        let hand4 = Hand::new_with_joker("QQQJA 123", true);
        let hand5 = Hand::new_with_joker("KTJJT 123", true);
        assert_eq!(hand3.win, Win::FourOfAKind(Rank::Five));
        assert_eq!(hand4.win, Win::FourOfAKind(Rank::Queen));
        assert_eq!(hand5.win, Win::FourOfAKind(Rank::Ten));

        assert!(hand3 < hand4);
        assert!(hand4 < hand5);
        assert!(hand3 < hand5);

        let hand6 = Hand::new_with_joker("JJJJJ 123", true);
        let hand7 = Hand::new_with_joker("QQQQ2 123", true);
        assert!(hand6 > hand7);

        let hand8 = Hand::new_with_joker("AJJJJ 123", true);
        assert_eq!(hand8.win, Win::FiveOfAKind(Rank::Ace));

        let hand9 = Hand::new_with_joker("JJ4QK 123", true);
        assert_eq!(hand9.win, Win::ThreeOfAKind(Rank::King));
    }
}
