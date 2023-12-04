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
            .map(|w| w.parse::<u32>().unwrap())
            .collect();
        let numbers: HashSet<_> = whitespace_re
            .split(numbers_str.trim())
            .map(|n| n.parse::<u32>().unwrap())
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
