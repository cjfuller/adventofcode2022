use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

use adventofcode2022::read_input_lines_as;

type Pos = (i32, i32);

#[derive(Debug, Default)]
struct Rope {
    knots: Vec<Pos>,
    tail_history: HashSet<Pos>,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Self::L,
            "R" => Self::R,
            "U" => Self::U,
            "D" => Self::D,
            _ => panic!("Unknown direction {s}"),
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    dir: Direction,
    num: i32,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        Ok(Instruction {
            dir: parts[0].parse().unwrap(),
            num: parts[1].parse().unwrap(),
        })
    }
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut value = Self::default();
        for _ in 0..size {
            value.knots.push((0, 0));
        }
        value
    }
    fn apply(&mut self, d: Instruction) {
        use Direction::*;
        //println!("{d:?}, start: h{:?} t{:?}", self.head, self.tail);
        self.tail_history.insert(*self.knots.last().unwrap());
        if d.num == 0 {
            return;
        }
        match d.dir {
            L => self.knots[0].0 -= 1,
            R => self.knots[0].0 += 1,
            U => self.knots[0].1 += 1,
            D => self.knots[0].1 -= 1,
        }
        self.update_tail_pos(1);
        self.tail_history.insert(*self.knots.last().unwrap());
        //println!("{d:?}, end: h{:?} t{:?}", self.head, self.tail);
        self.apply(Instruction {
            dir: d.dir,
            num: d.num - 1,
        });
    }
    fn update_tail_pos(&mut self, curr: usize) {
        if curr >= self.knots.len() {
            return;
        }
        let hp = curr - 1;
        let tp = curr;
        let diff = (
            self.knots[hp].0 - self.knots[tp].0,
            self.knots[hp].1 - self.knots[tp].1,
        );
        match diff {
            (n, 0) if n >= 2 => self.knots[tp].0 += 1,
            (n, 0) if n <= -2 => self.knots[tp].0 -= 1,
            (0, n) if n >= 2 => self.knots[tp].1 += 1,
            (0, n) if n <= -2 => self.knots[tp].1 -= 1,
            (x, y) if x.abs() > 1 || y.abs() > 1 => {
                self.knots[tp].0 += x / x.abs();
                self.knots[tp].1 += y / y.abs();
            }
            (_, _) => (),
        }
        self.update_tail_pos(curr + 1);
    }
}

fn part1() {
    let mut rope = Rope::new(2);
    let instructions: Vec<Instruction> = read_input_lines_as(9);
    for ins in instructions {
        rope.apply(ins);
    }
    println!("Part 1: {}", rope.tail_history.len());
}

fn part2() {
    let mut rope = Rope::new(10);
    let instructions: Vec<Instruction> = read_input_lines_as(9);
    for ins in instructions {
        rope.apply(ins);
    }
    println!("Part 1: {}", rope.tail_history.len());
}

fn main() {
    part1();
    part2();
}
