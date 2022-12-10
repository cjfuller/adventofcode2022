use std::collections::HashSet;

use adventofcode2022::read_input_lines;
use itertools::Itertools;

struct Rucksack {
    compartment_0: String,
    compartment_1: String,
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u32) - ('a' as u32) + 1
    } else {
        (c as u32) - ('A' as u32) + 27
    }
}

impl Rucksack {
    fn common_char(&self) -> char {
        for c0 in self.compartment_0.chars() {
            if self.compartment_1.contains(c0) {
                return c0;
            }
        }
        panic!("Found no common chars");
    }
}

fn part1() {
    let inputs = read_input_lines(3, false).into_iter().map(|line| {
        let (c0, c1) = line.split_at(line.len() / 2);
        Rucksack {
            compartment_0: c0.to_string(),
            compartment_1: c1.to_string(),
        }
    });
    let result: u32 = inputs.map(|it| it.common_char()).map(priority).sum();
    println!("Part 1: {result}");
}

fn find_common_char(elves: &[String]) -> char {
    let mut chars: HashSet<char> = HashSet::from_iter(elves[0].chars());
    for elf in elves.iter().skip(1) {
        chars = chars
            .intersection(&HashSet::from_iter(elf.chars()))
            .copied()
            .collect();
    }
    assert!(chars.len() == 1);
    *chars.iter().next().unwrap()
}

fn part2() {
    let inputs: Vec<Vec<String>> = read_input_lines(3, false)
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|it| it.collect())
        .collect();
    let result: u32 = inputs
        .iter()
        .map(|it| find_common_char(it))
        .map(priority)
        .sum();
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
