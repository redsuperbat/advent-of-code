use std::fs;

struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn contains(&self, r2: &Range) -> bool {
        self.low <= r2.low && self.high >= r2.high
    }

    fn overlaps_with(&self, r2: &Range) -> bool {
        self.high >= r2.low && self.low <= r2.high
    }

    fn from_str(string: &str) -> Range {
        let (low, high) = string.split_once('-').unwrap();
        Range {
            low: low.parse::<u32>().unwrap(),
            high: high.parse::<u32>().unwrap(),
        }
    }
}

fn part_one() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res: u32 = input
        .split('\n')
        .map(|it| it.split_once(',').unwrap())
        .map(|(first, second)| (Range::from_str(first), Range::from_str(second)))
        .map(|(first, second)| u32::from(first.contains(&second) || second.contains(&first)))
        .sum();

    println!("Part one result: {:?}", res)
}

fn part_two() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res: u32 = input
        .split('\n')
        .map(|it| it.split_once(',').unwrap())
        .map(|(first, second)| (Range::from_str(first), Range::from_str(second)))
        .map(|(first, second)| u32::from(first.overlaps_with(&second)))
        .sum();
    println!("Part two result: {:?}", res)
}

fn main() {
    part_one();
    part_two();
}
