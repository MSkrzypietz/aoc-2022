use std::fs;

enum GameResult {
    Loss,
    Draw,
    Win,
}

impl GameResult {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            'X' => Some(Self::Loss),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_opponent_char(c: &char) -> Option<Self> {
        match c {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn from_own_char(c: &char) -> Option<Self> {
        match c {
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn get_move_type_value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn calc_move_for_game_result(&self, gr: &GameResult) -> Self {
        match (self, gr) {
            (m, GameResult::Draw) => m.clone(),
            (Self::Rock, GameResult::Win) | (Self::Scissors, GameResult::Loss) => Self::Paper,
            (Self::Rock, GameResult::Loss) | (Self::Paper, GameResult::Win) => Self::Scissors,
            (Self::Paper, GameResult::Loss) | (Self::Scissors, GameResult::Win) => Self::Rock,
        }
    }

    fn play(&self, other: &Self) -> i32 {
        match (self, other) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => 3,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => 0,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => 6,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .collect::<Vec<(char, char)>>();

    let result1: i32 = input
        .iter()
        .map(|(opponent_move, own_move)| {
            (
                Move::from_opponent_char(&opponent_move).unwrap(),
                Move::from_own_char(&own_move).unwrap(),
            )
        })
        .map(|(opponent_move, own_move)| {
            own_move.play(&opponent_move) + own_move.get_move_type_value()
        })
        .sum();

    println!("Part 1: {}", result1);

    let result2: i32 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .map(|(opponent_move, game_result)| {
            (
                Move::from_opponent_char(&opponent_move).unwrap(),
                GameResult::from_char(&game_result).unwrap(),
            )
        })
        .map(|(opponent_move, game_result)| {
            let own_move = opponent_move.calc_move_for_game_result(&game_result);
            own_move.play(&opponent_move) + own_move.get_move_type_value()
        })
        .sum();

    println!("Part 2: {}", result2);
}
