mod test;
use std::collections::HashSet;

use itertools::Itertools;

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

#[derive(Debug)]
struct ElfGroup {
    rucksacks: Vec<Rucksack>,
    badge: char,
}

impl ElfGroup {
    fn new(rucksacks: Vec<Rucksack>) -> ElfGroup {
        let badge = *rucksacks
            .iter()
            .fold(HashSet::new(), |acc, x| match acc.len() {
                0 => x.set.clone(),
                _ => acc
                    .intersection(&x.set)
                    .map(|x| *x)
                    .collect::<HashSet<char>>(),
            })
            .iter()
            .map(|x| *x)
            .collect::<Vec<char>>()
            .first()
            .unwrap_or(&'a');
        ElfGroup { rucksacks, badge }
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    first_compartment: HashSet<char>,
    second_compartment: HashSet<char>,
    set: HashSet<char>,
}

impl Rucksack {
    fn intersection(&self) -> HashSet<char> {
        self.first_compartment
            .intersection(&self.second_compartment)
            .into_iter()
            .map(|x| *x)
            .collect()
    }

    fn nth_intersection(&self, index: usize) -> char {
        *self.intersection().iter().nth(index).unwrap_or(&'a')
    }
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let compartment_size = chars.len() / 2_usize;
        let first_compartment = chars
            .iter()
            .take(compartment_size)
            .map(|x| *x)
            .collect::<HashSet<char>>();
        let second_compartment = chars
            .iter()
            .rev()
            .take(compartment_size)
            .map(|x| *x)
            .collect::<HashSet<char>>();
        let set = chars.into_iter().collect::<HashSet<char>>();

        Rucksack {
            first_compartment,
            second_compartment,
            set,
        }
    }
}

pub fn solve_part_one(input: &str) -> String {
    let rucksacks = input
        .split("\n")
        .map(|x| Rucksack::from(x))
        .collect::<Vec<Rucksack>>();
    let sum: usize = rucksacks
        .iter()
        .map(|x| x.nth_intersection(0) as usize)
        .map(|x| match x {
            0..=90 => (x % 64) + 26,
            _ => x % 96,
        })
        .sum();

    format!("{}", sum)
}

pub fn solve_part_two(input: &str) -> String {
    let elf_groups: Vec<ElfGroup> = input
        .split("\n")
        .map(|x| Rucksack::from(x))
        .collect::<Vec<Rucksack>>()
        .chunks(3)
        .map(|x| ElfGroup::new(x.to_vec()))
        .collect();

    let sum: usize = elf_groups
        .iter()
        .map(|x| x.badge as usize)
        .map(|x| match x {
            0..=90 => (x % 64) + 26,
            _ => x % 96,
        })
        .sum();

    format!("{}", sum)
}
