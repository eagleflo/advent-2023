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

    println!("08 - Part One: {}", steps);
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
    let (instructions, net_str) = input.trim().split_once("\n\n").unwrap();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in net_str.lines() {
        let (node, pairstr) = line.split_once(" = (").unwrap();
        network.insert(node, pairstr[..pairstr.len() - 1].split_once(", ").unwrap());
    }

    let mut cursors: Vec<Vec<&str>> = vec![];
    for node in network.keys() {
        if node.ends_with('A') {
            cursors.push(vec![node]);
        }
    }

    let mut cycle_lengths = vec![];
    for (step, instruction) in instructions.chars().cycle().enumerate() {
        for cursor in &mut cursors {
            let mut next = cursor.last().unwrap();
            if next.ends_with('Z') {
                continue;
            }
            let pair = network[next];
            match instruction {
                'L' => next = &pair.0,
                'R' => next = &pair.1,
                _ => panic!("Unknown instruction"),
            }
            cursor.push(next);
            if next.ends_with('Z') {
                cycle_lengths.push(step + 1);
            }
        }
        if cycle_lengths.len() == cursors.len() {
            break;
        }
    }

    let steps = cycle_lengths
        .iter()
        .fold(1, |x, y| num::integer::lcm(x, *y));
    println!("08 - Part One: {}", steps);
}
