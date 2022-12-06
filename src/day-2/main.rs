use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code::NoOpResult;

#[derive(Eq, PartialEq)]
enum GameTossOption {
    Rock,
    Paper,
    Scissor,
}

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameTossOption {
    pub fn into_round_score(self) -> usize {
        match self {
            GameTossOption::Rock => 1,
            GameTossOption::Paper => 2,
            GameTossOption::Scissor => 3,
        }
    }

    pub fn determine_round_result(&self, other: GameTossOption) -> GameResult {
        match self {
            GameTossOption::Rock => match other {
                GameTossOption::Rock => GameResult::Draw,
                GameTossOption::Paper => GameResult::Loss,
                GameTossOption::Scissor => GameResult::Win,
            },
            GameTossOption::Paper => match other {
                GameTossOption::Rock => GameResult::Win,
                GameTossOption::Paper => GameResult::Draw,
                GameTossOption::Scissor => GameResult::Loss,
            },
            GameTossOption::Scissor => match other {
                GameTossOption::Rock => GameResult::Loss,
                GameTossOption::Paper => GameResult::Win,
                GameTossOption::Scissor => GameResult::Draw,
            },
        }
    }
}

fn game_result_from_toss(result: &str) -> Result<GameTossOption, String> {
    if result.eq("A") || result.eq("X") {
        return Ok(GameTossOption::Rock);
    } else if result.eq("B") || result.eq("Y") {
        return Ok(GameTossOption::Paper);
    } else if result.eq("C") || result.eq("Z") {
        return Ok(GameTossOption::Scissor);
    }

    Err("Game round contains unexpected input".to_owned())
}

/// Advent of Code day 2: https://adventofcode.com/2022/day/2
fn main() -> NoOpResult {
    // first, we'll open the input file and roll through it with a cursor
    let file = File::open("src/day-2/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_score = 0;

    for line in reader.lines() {
        let mut round_score = 0;
        let toss = line?;

        let toss_result: Vec<_> = toss.split(' ').collect();

        if toss_result.len() != 2 {
            continue;
        }

        let opponent_toss = game_result_from_toss(toss_result[0])?;
        let self_toss = game_result_from_toss(toss_result[1])?;

        let game_result = self_toss.determine_round_result(opponent_toss);

        match game_result {
            GameResult::Win => round_score += 6,
            GameResult::Loss => round_score += 0,
            GameResult::Draw => round_score += 3,
        }

        round_score += self_toss.into_round_score();
        total_score += round_score;
    }

    println!("total score is {}", total_score);

    Ok(())
}
