use std::fs;

fn main() {
    let input = fs::read("input.txt").unwrap();
    let calories = input
        .iter()
        .map(|it| *it as char)
        .collect::<String>()
        .split("\n\n")
        .map(|it| {
            it.split('\n')
                .map(|it| it.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .max()
        .unwrap();

    println!("{:?}", calories)
}
