use std::collections::HashSet;

use itertools::Itertools;
use rand::distributions::Slice;

mod test;

pub fn solve(input: &str) {
    let answer_part_one = solve_part_one(&input);
    let answer_part_two = solve_part_two(&input);
    println!(
        "Part 1 answer:{:#?}\nPart 2 answer:{:#?}",
        answer_part_one, answer_part_two
    )
}

fn parse_start_of_signal_marker(input: &str, window_size: usize) -> usize {
    let char_indicies = input.char_indices().collect::<Vec<(usize, char)>>();

    let windows = char_indicies.windows(window_size);

    for window in windows {
        let set = window.iter().map(|x| x.1).collect::<HashSet<char>>();
        if set.len() == window_size {
            return window.first().unwrap().0 + window_size;
        }
    }

    0

    // The below commented code seems fine, however is not. I'm really not sure why
    // this is and now am beyond caring.

    // let signal_start = windows
    //     .into_iter()
    //     .find(|x| {
    //         let set = x.iter().map(|x| x.1).collect::<HashSet<char>>();
    //         match set.len() {
    //             window_size => true,
    //             _ => false,
    //         }
    //     })
    //     .unwrap()
    //     .first()
    //     .unwrap();

    // signal_start.0 + window_size
}

pub fn solve_part_one(input: &str) -> String {
    let signal_start = parse_start_of_signal_marker(input, 4);

    format!("{}", signal_start)
}

pub fn solve_part_two(input: &str) -> String {
    let signal_start = parse_start_of_signal_marker(input, 14);

    format!("{}", signal_start)
}
