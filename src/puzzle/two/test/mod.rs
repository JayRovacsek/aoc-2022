#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUT: &str = "A Y
B X
C Z";

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("15"), solve_part_one(TEST_INPUT));
}

#[test]
fn test_rps() {
    assert_eq!(RockPaperScissors::from("A X").score_a, 4);
    assert_eq!(RockPaperScissors::from("A Y").score_a, 8);
    assert_eq!(RockPaperScissors::from("A Z").score_a, 3);
    assert_eq!(RockPaperScissors::from("B X").score_a, 1);
    assert_eq!(RockPaperScissors::from("B Y").score_a, 5);
    assert_eq!(RockPaperScissors::from("B Z").score_a, 9);
    assert_eq!(RockPaperScissors::from("C X").score_a, 7);
    assert_eq!(RockPaperScissors::from("C Y").score_a, 2);
    assert_eq!(RockPaperScissors::from("C Z").score_a, 6);

    assert_eq!(RockPaperScissors::from("A X").score_b, 3);
    assert_eq!(RockPaperScissors::from("A Y").score_b, 4);
    assert_eq!(RockPaperScissors::from("A Z").score_b, 8);
    assert_eq!(RockPaperScissors::from("B X").score_b, 1);
    assert_eq!(RockPaperScissors::from("B Y").score_b, 5);
    assert_eq!(RockPaperScissors::from("B Z").score_b, 9);
    assert_eq!(RockPaperScissors::from("C X").score_b, 2);
    assert_eq!(RockPaperScissors::from("C Y").score_b, 6);
    assert_eq!(RockPaperScissors::from("C Z").score_b, 7);
}

#[test]
fn test_solve_part_two() {
    assert_eq!(String::from("12"), solve_part_two(TEST_INPUT));
}
