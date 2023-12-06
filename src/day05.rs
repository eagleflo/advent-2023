use rayon::prelude::*;

fn parse_seeds(input: &str) -> Vec<u64> {
    let (_, number_str) = input.split_once(": ").unwrap();
    number_str
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug)]
struct Range {
    src: u64,
    dst: u64,
    rng: u64,
}

impl Range {
    pub fn contains(&self, number: u64) -> bool {
        number >= self.src && number < self.src + self.rng
    }

    pub fn transform(&self, number: u64) -> u64 {
        if self.src > self.dst {
            number - (self.src - self.dst)
        } else {
            number + (self.dst - self.src)
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    pub fn new(input: &[&str]) -> Self {
        let mut ranges = vec![];
        for line in input.iter().skip(1) {
            let numbers: Vec<u64> = line.split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
            let (dst, src, rng) = (numbers[0], numbers[1], numbers[2]);
            let range = Range { src, dst, rng };
            ranges.push(range);
        }
        Self { ranges }
    }

    pub fn get(&self, number: u64) -> u64 {
        if let Some(range) = self.ranges.iter().find(|r| r.contains(number)) {
            range.transform(number)
        } else {
            number
        }
    }
}

pub fn solve_part_one() {
    let input = std::fs::read_to_string("05.txt").unwrap();
    let blocks: Vec<Vec<_>> = input
        .trim()
        .split("\n\n")
        .map(|b| b.split('\n').collect::<Vec<_>>())
        .collect();
    let seeds: Vec<u64> = parse_seeds(blocks[0][0]);
    let maps = vec![
        Map::new(&blocks[1]),
        Map::new(&blocks[2]),
        Map::new(&blocks[3]),
        Map::new(&blocks[4]),
        Map::new(&blocks[5]),
        Map::new(&blocks[6]),
        Map::new(&blocks[7]),
    ];

    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        let soil = maps[0].get(seed);
        let fertilizer = maps[1].get(soil);
        let water = maps[2].get(fertilizer);
        let light = maps[3].get(water);
        let temperature = maps[4].get(light);
        let humidity = maps[5].get(temperature);
        let location = maps[6].get(humidity);
        locations.push(location);
    }
    let lowest = locations.iter().min().unwrap();

    println!("05 - Part One: {}", lowest);
}

fn parse_seed_ranges(input: &str) -> Vec<std::ops::Range<u64>> {
    let mut ranges: Vec<_> = vec![];
    let (_, number_str) = input.split_once(": ").unwrap();
    let numbers: Vec<u64> = number_str
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    for chunk in numbers.chunks(2) {
        let (src, rng) = (chunk[0], chunk[1]);
        ranges.push(src..src + rng);
    }
    ranges
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("05.txt").unwrap();
    let blocks: Vec<Vec<_>> = input
        .trim()
        .split("\n\n")
        .map(|b| b.split('\n').collect::<Vec<_>>())
        .collect();
    let seeds = parse_seed_ranges(blocks[0][0]);
    let maps = vec![
        Map::new(&blocks[1]),
        Map::new(&blocks[2]),
        Map::new(&blocks[3]),
        Map::new(&blocks[4]),
        Map::new(&blocks[5]),
        Map::new(&blocks[6]),
        Map::new(&blocks[7]),
    ];

    let mut min = std::u64::MAX;
    for range in seeds {
        let lowest = range
            .into_par_iter()
            .map(|seed| {
                let soil = maps[0].get(seed);
                let fertilizer = maps[1].get(soil);
                let water = maps[2].get(fertilizer);
                let light = maps[3].get(water);
                let temperature = maps[4].get(light);
                let humidity = maps[5].get(temperature);
                maps[6].get(humidity) // Location
            })
            .min()
            .unwrap();
        if lowest < min {
            min = lowest;
        }
    }

    println!("05 - Part Two: {}", min);
}
