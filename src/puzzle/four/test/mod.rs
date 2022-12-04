#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("2"), solve_part_one(TEST_INPUT));
}

#[test]
fn test_solve_part_two() {
    assert_eq!(String::from("4"), solve_part_two(TEST_INPUT));
}
