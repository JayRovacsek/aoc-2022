#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const ALT_TEST_INPUT_1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const ALT_TEST_INPUT_2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const ALT_TEST_INPUT_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const ALT_TEST_INPUT_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

#[test]
fn test_parse_start_of_signal_marker_four() {
    assert_eq!(
        String::from("7"),
        format!("{}", parse_start_of_signal_marker(TEST_INPUT, 4))
    );
    assert_eq!(
        String::from("5"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_1, 4))
    );
    assert_eq!(
        String::from("6"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_2, 4))
    );
    assert_eq!(
        String::from("10"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_3, 4))
    );
    assert_eq!(
        String::from("11"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_4, 4))
    );
}

#[test]
fn test_parse_start_of_signal_marker_fourteen() {
    assert_eq!(
        String::from("19"),
        format!("{}", parse_start_of_signal_marker(TEST_INPUT, 14))
    );
    assert_eq!(
        String::from("23"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_1, 14))
    );
    assert_eq!(
        String::from("23"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_2, 14))
    );
    assert_eq!(
        String::from("29"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_3, 14))
    );
    assert_eq!(
        String::from("26"),
        format!("{}", parse_start_of_signal_marker(ALT_TEST_INPUT_4, 14))
    );
}

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("7"), solve_part_one(TEST_INPUT));
}

#[test]
fn test_solve_part_two() {
    assert_eq!(String::from("19"), solve_part_two(TEST_INPUT));
}
