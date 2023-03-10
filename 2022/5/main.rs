use std::fs;

use regex::Regex;

fn init_state(s: &str) -> [String; 9] {
    let mut state: [String; 9] = Default::default();

    for it in s.lines() {
        let mut index = 0;
        let mut line = it;
        loop {
            let crate_or_number = line.get(..3).unwrap();
            if crate_or_number.trim().len() == 1 {
                break;
            }

            if crate_or_number != "   " {
                let char = crate_or_number.chars().nth(1).unwrap();
                state[index].push(char);
            }
            index += 1;

            line = match line.get(4..) {
                Some(val) => val,
                None => "",
            };

            if line.is_empty() {
                break;
            }
        }
    }
    state
}

fn parse_line(str: &str) -> (usize, usize, usize) {
    let re = Regex::new(r"\d+").unwrap();
    let vec = re
        .find_iter(str)
        .map(|m| m.as_str().parse().unwrap())
        .collect::<Vec<usize>>();
    (vec[0], vec[1] - 1, vec[2] - 1)
}

fn run_sim_1(state: &mut [String; 9], instruction: &str) {
    let (amount, from, to) = parse_line(instruction);
    let m_boxes = state[from].get(0..amount).unwrap().to_string();
    state[from] = state[from].get(amount..).unwrap().to_string();

    for char in m_boxes.chars() {
        state[to].insert_str(0, &char.to_string())
    }
}

fn run_sim_2(state: &mut [String; 9], instruction: &str) {
    let (amount, from, to) = parse_line(instruction);
    let m_boxes = state[from].get(0..amount).unwrap().to_string();
    state[from] = state[from].get(amount..).unwrap().to_string();

    // Part 2
    state[to].insert_str(0, &m_boxes);
}

fn part_one() -> String {
    let input = fs::read_to_string("input.txt").unwrap();
    let (init_string, program_instructions) = input.split_once("\n\n").unwrap();

    let mut state = init_state(init_string);
    for (idx, line) in program_instructions.lines().enumerate() {
        print!("Iteration: {} value: {:?} -> ", idx + 1, state);
        run_sim_1(&mut state, line);
        println!("{:?}", state);
    }
    state
        .iter()
        .map(|it| it.chars().next().unwrap())
        .collect::<String>()
}

fn part_two() -> String {
    let input = fs::read_to_string("input.txt").unwrap();
    let (init_string, program_instructions) = input.split_once("\n\n").unwrap();

    let mut state = init_state(init_string);
    for (idx, line) in program_instructions.lines().enumerate() {
        print!("Iteration: {} value: {:?} -> ", idx + 1, state);
        run_sim_2(&mut state, line);
        println!("{:?}", state);
    }
    state
        .iter()
        .map(|it| it.chars().next().unwrap())
        .collect::<String>()
}

fn main() {
    let part1 = part_one();
    let part2 = part_two();
    println!("Answer part 1: {}, 2: {}", part1, part2)
}
