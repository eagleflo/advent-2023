use regex::Regex;

struct Coord {
    row: usize,
    col: usize,
}

struct PartNumber {
    coord: Coord,
    span: usize,
    number: u32,
}

impl PartNumber {
    // Generate the perimeter given a number in a grid of given size
    pub fn perimeter(&self, width: usize, height: usize) -> Vec<Coord> {
        let mut coords: Vec<Coord> = vec![];
        let above = self.coord.row as isize - 1;
        let below = self.coord.row as isize + 1;
        let left = self.coord.col as isize - 1;
        let right = self.coord.col as isize + self.span as isize;

        if above >= 0 {
            for x in left..right + 1 {
                if x >= 0 && x < width as isize {
                    coords.push(Coord {
                        row: above as usize,
                        col: x as usize,
                    });
                }
            }
        }

        if left >= 0 {
            coords.push(Coord {
                row: self.coord.row,
                col: left as usize,
            });
        }

        if right < width as isize {
            coords.push(Coord {
                row: self.coord.row,
                col: right as usize,
            })
        }

        if below < height as isize {
            for x in left..right + 1 {
                if x >= 0 && x < width as isize {
                    coords.push(Coord {
                        row: below as usize,
                        col: x as usize,
                    });
                }
            }
        }

        coords
    }
}

pub fn solve_part_one() {
    let input = std::fs::read_to_string("03.txt").unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut numbers: Vec<PartNumber> = vec![];
    let digits_re = Regex::new(r"(\d+)+").unwrap();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();

    for (index, line) in input.trim().lines().enumerate() {
        grid.push(line.chars().collect());
        let matches: Vec<_> = digits_re.find_iter(line).collect();
        for m in matches {
            numbers.push(PartNumber {
                coord: Coord {
                    row: index,
                    col: m.start(),
                },
                span: m.end() - m.start(),
                number: m.as_str().parse().unwrap(),
            })
        }
    }

    let sum = numbers
        .iter()
        .filter(|n| {
            n.perimeter(width, height)
                .iter()
                .any(|c| grid[c.row][c.col] != '.')
        })
        .fold(0, |sum, n| sum + n.number);

    println!("03 - Part One: {}", sum);
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("03.txt").unwrap();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut numbers: Vec<PartNumber> = vec![];
    let mut gears: Vec<Coord> = vec![];
    let digits_re = Regex::new(r"(\d+)+").unwrap();
    let gear_re = Regex::new(r"(\*)").unwrap();
    let width = input.find('\n').unwrap();
    let height = input.lines().count();

    for (index, line) in input.trim().lines().enumerate() {
        grid.push(line.chars().collect());
        let matches: Vec<_> = digits_re.find_iter(line).collect();
        for m in matches {
            numbers.push(PartNumber {
                coord: Coord {
                    row: index,
                    col: m.start(),
                },
                span: m.end() - m.start(),
                number: m.as_str().parse().unwrap(),
            })
        }

        let gear_matches: Vec<_> = gear_re.find_iter(line).collect();
        for g in gear_matches {
            gears.push(Coord {
                row: index,
                col: g.start(),
            })
        }
    }

    let mut sum = 0;
    for gear in gears {
        let connected: Vec<_> = numbers
            .iter()
            .filter(|n| {
                n.perimeter(width, height)
                    .iter()
                    .any(|c| c.row == gear.row && c.col == gear.col)
            })
            .collect();
        if connected.len() == 2 {
            sum += connected[0].number * connected[1].number;
        }
    }

    println!("03 - Part Two: {}", sum);
}
