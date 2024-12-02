use std::collections::HashMap;
use std::io::BufRead;

pub fn part_1() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_1.txt").unwrap());

    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<usize> = Vec::new();

    reader.lines().map(|l| l.unwrap()).for_each(|line| {
        let mut sw = line.split_whitespace();
        lhs.push(sw.next().unwrap().parse::<usize>().unwrap());
        rhs.push(sw.next().unwrap().parse::<usize>().unwrap());
    });
    lhs.sort();
    rhs.sort();

    lhs.iter().zip(rhs).map(|(l, r)| l.abs_diff(r)).sum()
}

pub fn part_2() -> usize {
    let reader = std::io::BufReader::new(std::fs::File::open("input/day_1.txt").unwrap());
    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: HashMap<usize, usize> = HashMap::new();

    reader.lines().map(|l| l.unwrap()).for_each(|line| {
        let mut sw = line.split_whitespace();
        lhs.push(sw.next().unwrap().parse::<usize>().unwrap());
        *rhs.entry(sw.next().unwrap().parse::<usize>().unwrap())
            .or_insert(0) += 1;
    });

    lhs.iter().map(|l| l * *rhs.entry(*l).or_default()).sum()
}
