use phf::phf_map;
use std::fs;

enum GameInput {
    Rock(),
    Paper(),
    Scissors(),
}

impl GameInput {
    fn battle_against(&self, opponent: &GameInput) -> u32 {
        match self {
            GameInput::Rock() => match opponent {
                GameInput::Rock() => 3 + 1,
                GameInput::Paper() => 1,
                GameInput::Scissors() => 6 + 1,
            },
            GameInput::Paper() => match opponent {
                GameInput::Rock() => 2,
                GameInput::Paper() => 3 + 2,
                GameInput::Scissors() => 6 + 2,
            },
            GameInput::Scissors() => match opponent {
                GameInput::Rock() => 3,
                GameInput::Paper() => 6 + 3,
                GameInput::Scissors() => 3 + 3,
            },
        }
    }
}

static MAP: phf::Map<&'static str, GameInput> = phf_map! {
    "A" => GameInput::Rock(),
    "B" => GameInput::Paper(),
    "C" => GameInput::Scissors(),
    "Y" => GameInput::Rock(),
    "X" => GameInput::Paper(),
    "Z" => GameInput::Scissors()
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = input
        .split('\n')
        .map(|it| it.split_once(' ').unwrap())
        .map(|it| {
            let (a, b) = (MAP.get(it.0).unwrap(), MAP.get(it.1).unwrap());
            println!("Battling {} vs {}", it.0, it.1);
            let outcome = b.battle_against(a);
            println!("Outcome is {}", outcome);
            outcome
        })
        .sum::<u32>();
    println!("{}", result)
}
