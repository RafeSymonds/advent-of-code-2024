use std::io::BufRead;

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_2.txt").unwrap());
    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| {
            let row: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            row.windows(3).all(|w| {
                let diff1 = w[1].abs_diff(w[0]);
                let diff2 = w[2].abs_diff(w[1]);
                diff1 < 4
                    && diff1 > 0
                    && diff2 < 4
                    && diff2 > 0
                    && ((w[2] > w[1] && w[1] > w[0]) || (w[2] < w[1] && w[1] < w[0]))
            })
        })
        .count()
}
pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_2.txt").unwrap());
    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|line| {
            let row: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (0..row.len()).any(|i| {
                let subset: Vec<_> = row
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != i)
                    .map(|(_, &value)| value)
                    .collect();

                subset.windows(3).all(|w| {
                    let diff1 = w[1].abs_diff(w[0]);
                    let diff2 = w[2].abs_diff(w[1]);
                    diff1 < 4
                        && diff1 > 0
                        && diff2 < 4
                        && diff2 > 0
                        && ((w[2] > w[1] && w[1] > w[0]) || (w[2] < w[1] && w[1] < w[0]))
                })
            })
        })
        .count()
}
