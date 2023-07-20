mod game_options;
mod part_one;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::game_options::{get_game_result_from_toss, GameResult};

const INPUT_PATH: &str = "../inputs/day-2/input.txt";

/// Advent of Code day 2: https://adventofcode.com/2022/day/2
fn main() -> anyhow::Result<()> {
    // first, we'll open the input file and roll through it with a cursor
    let file = File::open(INPUT_PATH)?;
    let reader = BufReader::new(file);
    let mut total_score = 0;

    for line in reader.lines() {
        let mut round_score = 0;
        let toss = line?;

        let toss_result: Vec<_> = toss.split(' ').collect();

        if toss_result.len() != 2 {
            continue;
        }

        let opponent_toss = get_game_result_from_toss(toss_result[0])?;
        let self_toss = get_game_result_from_toss(toss_result[1])?;

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
