pub fn solve_part_one() {
    let input = std::fs::read_to_string("01.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let first = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let last = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let combined =
            String::from_utf8(vec![line.as_bytes()[first], line.as_bytes()[last]]).unwrap();
        let number = combined.parse::<u32>().unwrap();
        sum += number;
    }

    println!("01 - Part One: {}", sum);
}

fn parse_slice(slice: &str, f: fn(&str, &str) -> bool) -> Option<char> {
    if f(slice, "one") {
        Some('1')
    } else if f(slice, "two") {
        Some('2')
    } else if f(slice, "three") {
        Some('3')
    } else if f(slice, "four") {
        Some('4')
    } else if f(slice, "five") {
        Some('5')
    } else if f(slice, "six") {
        Some('6')
    } else if f(slice, "seven") {
        Some('7')
    } else if f(slice, "eight") {
        Some('8')
    } else if f(slice, "nine") {
        Some('9')
    } else {
        None
    }
}

fn find_first(input: &str) -> Option<char> {
    for index in 0..(input.len()) {
        let c = input.chars().nth(index)?;
        if c.is_ascii_digit() {
            return Some(c);
        }

        let end = std::cmp::min(index + 5, input.len());
        let slice = &input[index..end];
        let numeral = parse_slice(slice, |x: &str, y: &str| str::starts_with(x, y));
        if let Some(c) = numeral {
            return Some(c);
        }
    }

    None
}

fn find_last(input: &str) -> Option<char> {
    for index in (0..(input.len())).rev() {
        let c = input.chars().nth(index)?;
        if c.is_ascii_digit() {
            return Some(c);
        }

        let start = std::cmp::max(index as isize - 4, 0) as usize;
        let slice = &input[start..index + 1];
        let numeral = parse_slice(slice, |x: &str, y: &str| str::ends_with(x, y));
        if let Some(c) = numeral {
            return Some(c);
        }
    }

    None
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("01.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let first = find_first(line).unwrap();
        let last = find_last(line).unwrap();
        let combined = format!("{}{}", first, last);
        let number = combined.parse::<u32>().unwrap();
        sum += number;
    }

    println!("01 - Part Two: {}", sum);
}
