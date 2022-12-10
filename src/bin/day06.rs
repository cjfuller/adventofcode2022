use std::collections::{HashSet, VecDeque};

use adventofcode2022::read_input;

fn find_start_of_uniq_seq(chars: &[char], n: usize) -> usize {
    let mut buf: VecDeque<char> = VecDeque::new();

    for (i, c) in chars.iter().enumerate() {
        if buf.len() < n {
            buf.push_back(*c);
        } else {
            let uniq: HashSet<char> = HashSet::from_iter(buf.iter().copied());
            if uniq.len() == n {
                return i;
            }

            buf.pop_front();
            buf.push_back(*c);
        }
    }

    panic!("didn't find a start of unique sequence");
}

fn part1() {
    let chars: Vec<char> = read_input(6).trim().chars().collect();
    let result = find_start_of_uniq_seq(&chars, 4);
    println!("Part 1: {result}");
}

fn part2() {
    let chars: Vec<char> = read_input(6).trim().chars().collect();
    let result = find_start_of_uniq_seq(&chars, 14);
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
