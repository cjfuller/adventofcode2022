use std::convert::Infallible;
use std::str::FromStr;

use adventofcode2022::read_input_lines;

fn initial_stacks() -> Vec<Vec<char>> {
    vec![
        vec!['D', 'H', 'N', 'Q', 'T', 'W', 'V', 'B'],
        vec!['D', 'W', 'B'],
        vec!['T', 'S', 'Q', 'W', 'J', 'C'],
        vec!['F', 'J', 'R', 'N', 'Z', 'T', 'P'],
        vec!['G', 'P', 'V', 'J', 'M', 'S', 'T'],
        vec!['B', 'W', 'F', 'T', 'N'],
        vec!['B', 'L', 'D', 'Q', 'F', 'H', 'V', 'N'],
        vec!['H', 'P', 'F', 'R'],
        vec!['Z', 'S', 'M', 'B', 'L', 'N', 'P', 'H'],
    ]
}

struct Instruction {
    quantity: u32,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        Ok(Instruction {
            quantity: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
        })
    }
}

impl Instruction {
    fn apply_9000(&self, to: &mut [Vec<char>]) {
        for _i in 0..self.quantity {
            let curr = to[self.from - 1].pop().unwrap();
            to[self.to - 1].push(curr);
        }
    }

    fn apply_9001(&self, to: &mut [Vec<char>]) {
        let mut buf = vec![];
        for _i in 0..self.quantity {
            let curr = to[self.from - 1].pop().unwrap();
            buf.push(curr);
        }

        for _i in 0..self.quantity {
            to[self.to - 1].push(buf.pop().unwrap());
        }
    }
}

fn read_instructions() -> Vec<Instruction> {
    read_input_lines(5, false)
        .into_iter()
        .filter(|it| it.starts_with("move"))
        .map(|it| it.parse().unwrap())
        .collect()
}

fn form_top_string(stacks: &[Vec<char>]) -> String {
    stacks.iter().map(|it| it.last().unwrap()).collect()
}

fn part1() {
    let mut stacks = initial_stacks();
    let instructions = read_instructions();
    for ins in instructions {
        ins.apply_9000(&mut stacks);
    }
    println!("Part 1: {}", form_top_string(&stacks))
}

fn part2() {
    let mut stacks = initial_stacks();
    let instructions = read_instructions();
    for ins in instructions {
        ins.apply_9001(&mut stacks);
    }
    println!("Part 2: {}", form_top_string(&stacks))
}

fn main() {
    part1();
    part2();
}
