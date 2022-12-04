use std::collections::HashSet;

use itertools::Itertools;

mod test;

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

#[derive(Debug)]
struct CleaningCrew {
    first: Vec<u32>,
    first_hash: HashSet<u32>,
    second: Vec<u32>,
    second_hash: HashSet<u32>,
}

fn contains_range(target: &Vec<u32>, start: &u32, end: &u32) -> bool {
    target.contains(start) && target.contains(end)
}

impl CleaningCrew {
    fn has_redundancy(&self) -> bool {
        match (self.first.len() > self.second.len()) {
            true => contains_range(
                &self.first,
                self.second.first().unwrap(),
                self.second.last().unwrap(),
            ),
            false => contains_range(
                &self.second,
                self.first.first().unwrap(),
                self.first.last().unwrap(),
            ),
        }
    }

    fn has_overlap(&self) -> bool {
        self.first_hash
            .intersection(&self.second_hash)
            .collect::<Vec<&u32>>()
            .len()
            != 0
    }
}

impl From<&str> for CleaningCrew {
    fn from(input: &str) -> Self {
        let parts = input.split(',').collect::<Vec<&str>>();
        let first_segments = parts.first().unwrap_or(&"0-0").split('-').collect_vec();
        let second_segments = parts
            .iter()
            .nth(1)
            .unwrap_or(&"0-0")
            .split('-')
            .collect_vec();
        let first = (first_segments
            .first()
            .unwrap_or(&"0")
            .parse::<u32>()
            .unwrap_or(0_u32)
            ..=first_segments
                .iter()
                .nth(1)
                .unwrap_or(&"0")
                .parse::<u32>()
                .unwrap_or(0_u32))
            .collect::<Vec<u32>>();

        let second = (second_segments
            .first()
            .unwrap_or(&"0")
            .parse::<u32>()
            .unwrap_or(0_u32)
            ..=second_segments
                .iter()
                .nth(1)
                .unwrap_or(&"0")
                .parse::<u32>()
                .unwrap_or(0_u32))
            .collect::<Vec<u32>>();

        let first_hash = first.iter().map(|x| *x).collect::<HashSet<u32>>();
        let second_hash = second.iter().map(|x| *x).collect::<HashSet<u32>>();

        CleaningCrew {
            first,
            second,
            first_hash,
            second_hash,
        }
    }
}

pub fn solve_part_one(input: &str) -> String {
    let cleaning_crews: Vec<CleaningCrew> =
        input.split("\n").map(|x| CleaningCrew::from(x)).collect();

    let redundant_crews = cleaning_crews
        .iter()
        .map(|x| x.has_redundancy())
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len();

    format!("{}", redundant_crews)
}

pub fn solve_part_two(input: &str) -> String {
    let cleaning_crews: Vec<CleaningCrew> =
        input.split("\n").map(|x| CleaningCrew::from(x)).collect();

    let overlap_crews = cleaning_crews
        .iter()
        .map(|x| x.has_overlap())
        .filter(|x| *x)
        .collect::<Vec<bool>>()
        .len();

    format!("{}", overlap_crews)
}
