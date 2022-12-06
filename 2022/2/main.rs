use phf::phf_map;
use std::fs;

#[derive(Debug)]
enum GameInput {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl GameInput {
    fn battle_against(&self, opponent: &GameInput) -> u32 {
        match self {
            GameInput::Rock => match opponent {
                GameInput::Rock => 3 + 1,
                GameInput::Paper => 1,
                GameInput::Scissors => 6 + 1,
            },
            GameInput::Paper => match opponent {
                GameInput::Rock => 6 + 2,
                GameInput::Paper => 3 + 2,
                GameInput::Scissors => 2,
            },
            GameInput::Scissors => match opponent {
                GameInput::Rock => 3,
                GameInput::Paper => 6 + 3,
                GameInput::Scissors => 3 + 3,
            },
        }
    }
}

impl Outcome {
    fn get_required_input(&self, match_against: &GameInput) -> GameInput {
        match self {
            Outcome::Win => match match_against {
                GameInput::Rock => GameInput::Paper,
                GameInput::Paper => GameInput::Scissors,
                GameInput::Scissors => GameInput::Rock,
            },
            Outcome::Loss => match match_against {
                GameInput::Rock => GameInput::Scissors,
                GameInput::Paper => GameInput::Rock,
                GameInput::Scissors => GameInput::Paper,
            },
            Outcome::Draw => match match_against {
                GameInput::Rock => GameInput::Rock,
                GameInput::Paper => GameInput::Paper,
                GameInput::Scissors => GameInput::Scissors,
            },
        }
    }
}

static PART_ONE_MAP: phf::Map<&'static str, GameInput> = phf_map! {
    "A" => GameInput::Rock,
    "B" => GameInput::Paper,
    "C" => GameInput::Scissors,
    "X" => GameInput::Rock,
    "Y" => GameInput::Paper,
    "Z" => GameInput::Scissors
};

static PART_TWO_MAP: phf::Map<&'static str, Outcome> = phf_map! {
    "X" => Outcome::Loss,
    "Y" => Outcome::Draw,
    "Z" => Outcome::Win
};

fn main() {
    // Part one
    let input = fs::read_to_string("input.txt").unwrap();
    let result = input
        .split('\n')
        .map(|it| it.split_once(' ').unwrap())
        .map(|it| {
            let (opponent, me) = (
                PART_ONE_MAP.get(it.0).unwrap(),
                PART_ONE_MAP.get(it.1).unwrap(),
            );
            me.battle_against(opponent)
        })
        .sum::<u32>();
    println!("Part one: {}", result);

    // Part two
    let result = input
        .split('\n')
        .map(|it| it.split_once(' ').unwrap())
        .map(|it| {
            let (opponent, determined_outcome) = (
                PART_ONE_MAP.get(it.0).unwrap(),
                PART_TWO_MAP.get(it.1).unwrap(),
            );
            println!(
                "Opponent {:?}, Determined outcome {:?}",
                opponent, determined_outcome
            );
            let me = determined_outcome.get_required_input(opponent);
            println!("My input {:?}", me);
            me.battle_against(opponent)
        })
        .sum::<u32>();
    println!("Part two: {}", result);
}
