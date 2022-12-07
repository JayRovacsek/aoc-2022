#![allow(unused)]
use super::*;
use crate::input::read_contents;

const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[test]
fn test_solve_part_one() {
    assert_eq!(String::from("95437"), solve_part_one(TEST_INPUT));
}

#[test]
fn test_solve_part_two() {
    assert_eq!(
        String::from("This is a stub"),
        solve_part_two("This is a stub")
    );
}
