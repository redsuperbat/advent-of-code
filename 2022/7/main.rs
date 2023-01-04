use std::{collections::HashMap, fs};

type State = (Vec<String>, HashMap<String, usize>);

fn parse_line(state: &mut State, cmd: &str) {
    if cmd == "$ ls" || cmd.starts_with("dir") {
        return;
    }

    let file_size = cmd.split_once(' ').unwrap().0.parse::<usize>();
    if file_size.is_ok() {
        let size = Result::unwrap(file_size);
        for (i, dir) in state.0.iter().enumerate() {
            let key = state.0[..(i + 1)].join("/");
            let dir_size = state.1.get(dir).unwrap_or(&0);
            println!("Adding {} to dir {} State: {:?}", size, key, state.0);
            state.1.insert(key, dir_size + size);
        }
        return;
    }

    if cmd == "$ cd .." {
        state.0.pop();
        let current_dir = state.0.join("/");
        println!("Moved one directory up!");
        println!("pwd: {}", &current_dir);
        return;
    }

    if cmd == "$ cd /" {
        state.0 = vec![String::from("/")];
        return;
    }

    if cmd.starts_with("$ cd") {
        let dir = cmd.get("$ cd ".len()..).unwrap();
        println!("Changing dir to {}", dir);
        state.0.push(dir.to_string());
        let current_dir = state.0.join("/");
        println!("pwd: {}", &current_dir);
    }
}

fn part_one() -> usize {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut state: State = (vec![], HashMap::new());
    for line in input.lines() {
        parse_line(&mut state, line);
    }
    let eligible_dirs = state
        .1
        .iter()
        .filter(|(_, &v)| v <= 100_000)
        .map(|(_, v)| *v)
        .collect::<Vec<_>>();

    println!(
        "{}/{} dirs were below 100_000 bytes",
        eligible_dirs.len(),
        state.1.len()
    );

    eligible_dirs.iter().sum()
}

fn part_two() -> String {
    "".to_string()
}

fn main() {
    let part1 = part_one();
    let part2 = part_two();
    println!("Answer part 1: {}, 2: {}", part1, part2)
}
