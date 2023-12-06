pub fn solve_part_one() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let (idstr, cubes) = line.split_once(": ").unwrap();
        let id = idstr[5..].parse::<u8>().unwrap();
        let mut possible = true;

        for c in cubes.split(&[',', ';']) {
            let (numberstr, color) = c.trim().split_once(' ').unwrap();
            let number = numberstr.parse::<u8>().unwrap();
            match color {
                "red" => {
                    if number > 12 {
                        possible = false
                    }
                }
                "green" => {
                    if number > 13 {
                        possible = false
                    }
                }
                "blue" => {
                    if number > 14 {
                        possible = false
                    }
                }
                _ => (),
            }
        }

        if possible {
            sum += id as u32
        }
    }

    println!("02 - Part One: {}", sum);
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let (_, cubes) = line.split_once(": ").unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for c in cubes.split(&[',', ';']) {
            let (numberstr, color) = c.trim().split_once(' ').unwrap();
            let number = numberstr.parse::<u32>().unwrap();
            match color {
                "red" => {
                    if number > red {
                        red = number
                    }
                }
                "green" => {
                    if number > green {
                        green = number
                    }
                }
                "blue" => {
                    if number > blue {
                        blue = number
                    }
                }
                _ => (),
            }
        }

        let power = red * green * blue;
        sum += power;
    }

    println!("02 - Part Two: {}", sum);
}
