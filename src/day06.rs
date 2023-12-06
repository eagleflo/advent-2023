use regex::Regex;
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn ways_to_win(&self) -> u64 {
        let mut wins = 0;
        for n in 0..self.time {
            let speed = n;
            let remaining = self.time - n;
            if speed * remaining > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

pub fn solve_part_one() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
    let digits_re = Regex::new(r"(\d+)").unwrap();
    let times: Vec<u64> = digits_re
        .find_iter(input.lines().next().unwrap())
        .map(|d| d.as_str().parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = digits_re
        .find_iter(input.lines().nth(1).unwrap())
        .map(|d| d.as_str().parse::<u64>().unwrap())
        .collect();
    let races: Vec<Race> = zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    let ways_to_win: Vec<u64> = races.iter().map(|race| race.ways_to_win()).collect();
    let total: u64 = ways_to_win.iter().product();
    println!("06 - Part One: {}", total);
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("input/06.txt").unwrap();
    let digits_re = Regex::new(r"(\d+)").unwrap();
    let time: u64 = digits_re
        .find_iter(input.lines().next().unwrap())
        .map(|d| d.as_str().to_string())
        .reduce(|cur: String, next: String| cur + &next)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let distance: u64 = digits_re
        .find_iter(input.lines().nth(1).unwrap())
        .map(|d| d.as_str().to_string())
        .reduce(|cur: String, next: String| cur + &next)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let race = Race { time, distance };
    println!("06 - Part Two: {}", race.ways_to_win());
}
