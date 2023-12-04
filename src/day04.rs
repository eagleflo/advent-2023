use regex::Regex;
use std::collections::HashSet;

pub fn solve_part_one() {
    let input = std::fs::read_to_string("04.txt").unwrap();
    let whitespace_re = Regex::new(r"\s+").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let (_, scratchcard) = line.split_once(": ").unwrap();
        let (winning_str, numbers_str) = scratchcard.split_once(" | ").unwrap();
        let mut score = 0;

        let winning: HashSet<_> = whitespace_re
            .split(winning_str.trim())
            .map(|w| w.parse::<u8>().unwrap())
            .collect();
        let numbers: HashSet<_> = whitespace_re
            .split(numbers_str.trim())
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        for _ in numbers.intersection(&winning) {
            if score == 0 {
                score += 1
            } else {
                score *= 2
            }
        }

        sum += score;
    }

    println!("04 - Part One: {}", sum);
}

#[derive(Clone, Debug)]
struct Card {
    id: u8,
    win_count: u8,
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("04.txt").unwrap();
    let number_re = Regex::new(r"(\d+)").unwrap();
    let whitespace_re = Regex::new(r"\s+").unwrap();
    let mut cards: Vec<Card> = vec![];

    for line in input.lines() {
        let (card_id_str, scratchcard) = line.split_once(": ").unwrap();
        let card_id = number_re
            .find(card_id_str)
            .unwrap()
            .as_str()
            .parse::<u8>()
            .unwrap();
        let (winning_str, numbers_str) = scratchcard.split_once(" | ").unwrap();

        let winning: HashSet<_> = whitespace_re
            .split(winning_str.trim())
            .map(|w| w.parse::<u8>().unwrap())
            .collect();
        let numbers: HashSet<_> = whitespace_re
            .split(numbers_str.trim())
            .map(|n| n.parse::<u8>().unwrap())
            .collect();

        let win_count = numbers.intersection(&winning).count() as u8;

        cards.push(Card {
            id: card_id,
            win_count,
        })
    }

    let mut additional_cards: Vec<Card> = vec![];
    let mut index = 0;
    loop {
        for card in &cards[index..] {
            if card.win_count > 0 {
                let pos = card.id as usize;
                let new = &cards[pos..pos + card.win_count as usize];
                additional_cards.extend_from_slice(new);
            }
            index += 1;
        }

        if additional_cards.is_empty() {
            break;
        }

        cards.extend_from_slice(&additional_cards);
        additional_cards.clear();
    }

    let sum = cards.len();
    println!("04 - Part Two: {}", sum);
}
