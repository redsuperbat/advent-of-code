use std::{fs, slice};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part_one() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = input
        .split('\n')
        .map(|it| {
            let half_length = it.len() / 2;
            (&it[..half_length], &it[half_length..])
        })
        .map(|(first, second)| {
            let char = first.chars().find(|it| second.contains(*it)).unwrap();
            let pos = LETTERS.chars().position(|it| it == char).unwrap() + 1;
            pos as u32
        })
        .sum::<u32>();
    println!("Result is {}", result)
}

fn part_two() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = input
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|it| (it[0], it[1], it[2]))
        .map(|(f, s, t)| {
            let char = f
                .chars()
                .find(|it| s.contains(*it) && t.contains(*it))
                .unwrap();
            let pos = LETTERS.chars().position(|it| it == char).unwrap() + 1;
            pos as u32
        })
        .sum::<u32>();
    println!("Result part two {}", result)
}

fn main() {
    part_one();
    part_two();
}
