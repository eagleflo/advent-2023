fn subseq(seq: &[i32]) -> Vec<i32> {
    seq.windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>()
}

pub fn solve_part_one() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
    let seqs = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|token| token.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for seq in seqs {
        let mut subseqs: Vec<Vec<i32>> = vec![seq];
        loop {
            let subseq = subseq(subseqs.last().unwrap());
            if subseq.iter().all(|n| *n == 0) {
                subseqs.push(subseq);
                break;
            }
            subseqs.push(subseq);
        }

        let next = subseqs.iter().rfold(0, |sum, ss| sum + ss.last().unwrap());
        sum += next;
    }

    println!("09 - Part One: {}", sum);
}

pub fn solve_part_two() {
    let input = std::fs::read_to_string("input/09.txt").unwrap();
    let seqs = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|token| token.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for mut seq in seqs {
        seq.reverse();
        let mut subseqs: Vec<Vec<i32>> = vec![seq];
        loop {
            let subseq = subseq(subseqs.last().unwrap());
            if subseq.iter().all(|n| *n == 0) {
                subseqs.push(subseq);
                break;
            }
            subseqs.push(subseq);
        }

        let next = subseqs.iter().rfold(0, |sum, ss| sum + ss.last().unwrap());
        sum += next;
    }

    println!("09 - Part Two: {}", sum);
}
