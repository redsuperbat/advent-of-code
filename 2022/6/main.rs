use std::{collections::HashSet, fs};

fn shared(packet_len: usize) -> String {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut window = String::new();

    for (i, char) in input.chars().enumerate() {
        println!("Iteration {}: {:?}", i, window);
        let w_len = window.len();

        if w_len == packet_len {
            let hs = window.chars().collect::<HashSet<_>>();
            if hs.len() == w_len {
                return i.to_string();
            }
            window.remove(0);
            window.push(char);
            continue;
        }
        window.push(char);
    }
    panic!("Bad algo :(")
}

fn part_one() -> String {
    shared(4)
}

fn part_two() -> String {
    shared(14)
}

fn main() {
    let part1 = part_one();
    let part2 = part_two();
    println!("Answer part 1: {}, 2: {}", part1, part2)
}
