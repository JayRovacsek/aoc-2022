#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("157"), solve_part_one(TEST_INPUT));
}

#[test]
fn test_solve_part_two() {
    assert_eq!(String::from("70"), solve_part_two(TEST_INPUT));
}
