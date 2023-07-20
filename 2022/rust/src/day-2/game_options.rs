#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameTossOption {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
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

pub fn get_game_result_from_toss(result: &str) -> anyhow::Result<GameTossOption> {
    if result.eq("A") || result.eq("X") {
        return Ok(GameTossOption::Rock);
    } else if result.eq("B") || result.eq("Y") {
        return Ok(GameTossOption::Paper);
    } else if result.eq("C") || result.eq("Z") {
        return Ok(GameTossOption::Scissor);
    }

    anyhow::bail!("Game round contains unexpected input".to_owned())
}
