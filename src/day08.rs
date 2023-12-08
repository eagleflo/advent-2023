use std::collections::HashMap;

pub fn solve_part_one() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
    let (instructions, net_str) = input.trim().split_once("\n\n").unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in net_str.lines() {
        let (node, pairstr) = line.split_once(" = (").unwrap();
        network.insert(node, pairstr[..pairstr.len() - 1].split_once(", ").unwrap());
    }

    let mut steps = 0;
    let mut next = "AAA";
    for instruction in instructions.chars().cycle() {
        let pair = network[next];
        match instruction {
            'L' => next = pair.0,
            'R' => next = pair.1,
            _ => panic!("Unknown instruction"),
        }
        steps += 1;
        if next == "ZZZ" {
            break;
        }
    }

    println!("06 - Part One: {}", steps);
}
