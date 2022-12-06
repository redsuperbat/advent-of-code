use std::fs;

fn get_max_and_remove(vec: &mut Vec<u32>) -> u32 {
    let max = *vec.iter().max().unwrap();
    let index = vec.iter().position(|it| *it == max).unwrap();
    vec.remove(index);
    max
}

fn main() {
    let input = fs::read("input.txt").unwrap();
    let mut calories = input
        .iter()
        .map(|it| *it as char)
        .collect::<String>()
        .split("\n\n")
        .filter(|it| !it.is_empty())
        .map(|it| {
            it.split('\n')
                .map(|it| it.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    let mut total: u32 = 0;
    for idx in 1..4 {
        let max = get_max_and_remove(&mut calories);
        total += max;
        println!("Max{}: {}", idx, max)
    }
    println!("Total: {}", total);
}
