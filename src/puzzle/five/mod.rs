use core::num;
use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use regex::Regex;

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
struct CrateState {
    state: HashMap<usize, Vec<char>>,
}

#[derive(Debug)]
struct Operation {
    start: usize,
    end: usize,
    count: usize,
}

impl CrateState {
    fn p1_apply_operations(&mut self, operations: &Vec<Operation>) {
        operations.iter().for_each(|x| self.p1_apply_operation(x))
    }

    fn p1_apply_operation(&mut self, operation: &Operation) {
        let to_move = self.take(&operation);
        let end = self.state.get_mut(&operation.end).unwrap();
        to_move.into_iter().for_each(|x| end.insert(0, x));
    }

    fn p2_apply_operations(&mut self, operations: &Vec<Operation>) {
        operations.iter().for_each(|x| self.p2_apply_operation(x))
    }

    fn p2_apply_operation(&mut self, operation: &Operation) {
        let to_move = self.take(&operation);
        let end = self.state.get_mut(&operation.end).unwrap();
        to_move.into_iter().rev().for_each(|x| end.insert(0, x));
    }

    fn take(&mut self, operation: &Operation) -> Vec<char> {
        let start = self.state.get_mut(&operation.start).unwrap();
        start.drain(0..operation.count).collect()
    }
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        let num_re = Regex::new(r"\d+").unwrap();
        let parts = input
            .split(' ')
            .into_iter()
            .filter(|x| num_re.is_match(*x))
            .map(|x| x.parse::<usize>().unwrap_or(0))
            .collect::<Vec<usize>>();
        let count = *parts.iter().nth(0).unwrap_or(&0);
        let start = *parts.iter().nth(1).unwrap_or(&0);
        let end = *parts.iter().nth(2).unwrap_or(&0);

        Operation { start, end, count }
    }
}

impl From<&str> for CrateState {
    fn from(input: &str) -> Self {
        let content_re = Regex::new(r"[A-Z]").unwrap();

        let state_parts: Vec<Vec<Vec<char>>> = input
            .split("\n")
            .filter(|x| content_re.is_match(*x))
            .into_iter()
            .map(|x| {
                x.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|x| x.filter(|x| x.is_alphabetic()).collect::<Vec<char>>())
                    .collect_vec()
            })
            .collect_vec();

        let reverse_state = state_parts
            .iter()
            .map(|x| {
                x.iter()
                    .enumerate()
                    .map(|(i, y)| {
                        y.iter().map(move |z| match z.is_alphabetic() {
                            true => (i + 1, z),
                            false => (i + 1, &'_'),
                        })
                    })
                    .flatten()
                    .collect::<Vec<(usize, &char)>>()
            })
            .flatten()
            .map(|x| (x.0, *x.1))
            .fold(
                HashMap::new(),
                |mut acc: HashMap<usize, Vec<char>>, (i, x)| match x {
                    '_' => acc,
                    _ => match acc.contains_key(&i) {
                        true => {
                            acc.insert(
                                i,
                                vec![vec![x], acc.get(&i).unwrap().clone().to_vec()].concat(),
                            );
                            acc
                        }
                        false => {
                            acc.insert(i, vec![x]);
                            acc
                        }
                    },
                },
            );

        let state = reverse_state
            .into_iter()
            .map(|x| (x.0, x.1.iter().rev().map(|x| *x).collect::<Vec<char>>()))
            .collect::<HashMap<usize, Vec<char>>>();

        CrateState { state }
    }
}

pub fn solve_part_one(input: &str) -> String {
    let parts = input.split("\n\n").into_iter().collect::<Vec<&str>>();
    let mut cs = CrateState::from(*parts.first().unwrap_or(&""));

    let operations = parts
        .last()
        .unwrap_or(&"")
        .split("\n")
        .into_iter()
        .map(|x| Operation::from(x))
        .collect::<Vec<Operation>>();

    cs.p1_apply_operations(&operations);

    let top = cs
        .state
        .iter()
        .sorted_by(|a, b| a.0.cmp(b.0))
        .map(|x| x.1.iter().nth(0).unwrap_or(&'a'))
        .collect::<String>();

    format!("{}", top)
}

pub fn solve_part_two(input: &str) -> String {
    let parts = input.split("\n\n").into_iter().collect::<Vec<&str>>();
    let mut cs = CrateState::from(*parts.first().unwrap_or(&""));

    let operations = parts
        .last()
        .unwrap_or(&"")
        .split("\n")
        .into_iter()
        .map(|x| Operation::from(x))
        .collect::<Vec<Operation>>();

    cs.p2_apply_operations(&operations);

    let top = cs
        .state
        .iter()
        .sorted_by(|a, b| a.0.cmp(b.0))
        .map(|x| x.1.iter().nth(0).unwrap_or(&'a'))
        .collect::<String>();

    format!("{}", top)
}
