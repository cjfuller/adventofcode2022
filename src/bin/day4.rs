use std::convert::Infallible;
use std::str::FromStr;

use adventofcode2022::read_input_lines_as;

#[derive(Clone, Copy, Debug)]
struct ElfRange {
    lower: u32,
    upper: u32,
}

impl FromStr for ElfRange {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        Ok(ElfRange {
            lower: parts[0].parse().unwrap(),
            upper: parts[1].parse().unwrap(),
        })
    }
}

impl ElfRange {
    fn contains(&self, other: &ElfRange) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &ElfRange) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
            || other.lower <= self.upper && other.upper >= self.lower
    }
}

#[derive(Clone, Copy, Debug)]
struct ElfPair {
    first: ElfRange,
    second: ElfRange,
}

impl FromStr for ElfPair {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        Ok(ElfPair {
            first: parts[0].parse().unwrap(),
            second: parts[1].parse().unwrap(),
        })
    }
}

impl ElfPair {
    fn redundant(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }
}

fn part1() {
    let input = read_input_lines_as::<ElfPair>(4);
    let result = input.iter().filter(|it| it.redundant()).count();
    println!("Part 1: {result}");
}

fn part2() {
    let input = read_input_lines_as::<ElfPair>(4);
    let result = input
        .iter()
        .filter(|it| it.first.overlaps(&it.second))
        .count();
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
