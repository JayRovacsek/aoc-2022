#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("CMZ"), solve_part_one(TEST_INPUT));
}

#[test]
fn test_solve_part_two() {
    assert_eq!(String::from("MCD"), solve_part_two(TEST_INPUT));
}
