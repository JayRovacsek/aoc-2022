mod test;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

#[derive(Debug)]
struct RockPaperScissors {
    player: char,
    opponent: char,
    score_a: i32,
    score_b: i32,
}

fn outcome_a(player: i32, opponent: i32) -> i32 {
    if (player == 3 && opponent == 1) {
        return 0;
    }
    if (player == 1 && opponent == 3) {
        return 6;
    }
    match (player - opponent) {
        0 => 3,
        1 => 6,
        _ => 0,
    }
}

fn outcome_b(player: i32, opponent: i32) -> i32 {
    match player {
        3 => 6 + (opponent % 3) + 1,
        2 => 3 + opponent,
        _ => match opponent {
            1 => 3,
            2 => 1,
            _ => 2,
        },
    }
}

impl From<&str> for RockPaperScissors {
    fn from(input: &str) -> Self {
        let plays: Vec<char> = input.chars().into_iter().filter(|x| *x != ' ').collect();

        let player = *plays.last().unwrap();
        let opponent = *plays.first().unwrap();
        let player_base_score = player as u32 % 87_u32;
        let opponent_base_score = opponent as u32 % 64_u32;

        let score_a = (player_base_score as i32)
            + outcome_a(player_base_score as i32, opponent_base_score as i32);

        let score_b = outcome_b(player_base_score as i32, opponent_base_score as i32);

        RockPaperScissors {
            player,
            opponent,
            score_a,
            score_b,
        }
    }
}

pub fn solve_part_one(input: &str) -> String {
    let games = input.split("\n").map(|x| RockPaperScissors::from(x));
    let total_score: i32 = games.map(|x| x.score_a).sum();
    format!("{}", total_score)
}

pub fn solve_part_two(input: &str) -> String {
    let games = input.split("\n").map(|x| RockPaperScissors::from(x));
    let total_score: i32 = games.map(|x| x.score_b).sum();
    format!("{}", total_score)
}
