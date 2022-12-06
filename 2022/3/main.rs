use std::fs;

fn part_one() {
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
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

fn main() {
    part_one()
}
