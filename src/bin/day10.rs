use std::convert::Infallible;
use std::str::FromStr;

use adventofcode2022::read_input_lines_as;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "noop" {
            Self::Noop
        } else {
            let parts = s.split_ascii_whitespace().collect_vec();
            assert!(parts[0] == "addx");
            Self::Add(parts[1].parse().unwrap())
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    cycle: u32,
    register: i32,
}

#[derive(Clone, Copy, Debug)]
enum EvalResult {
    Ongoing,
    Finished,
}

impl State {
    fn initial() -> State {
        State {
            cycle: 1,
            register: 1,
        }
    }
    fn signal_strength(&self) -> i32 {
        self.register * self.cycle as i32
    }
    fn apply(self, ins: Instruction) -> State {
        use Instruction::*;
        match ins {
            Noop => State {
                cycle: self.cycle + 1,
                ..self
            },
            Add(n) => State {
                cycle: self.cycle + 2,
                register: self.register + n,
            },
        }
    }

    fn state_during_cycle(&self, ins: &[Instruction], cycle: u32) -> (State, EvalResult) {
        let mut state = *self;
        let mut eval_result = EvalResult::Finished;

        for i in ins {
            if state.cycle >= std::cmp::max(cycle, 1) - 1 {
                eval_result = EvalResult::Ongoing;
                break;
            }
            state = state.apply(*i);
        }

        state.cycle = cycle;
        (state, eval_result)
    }

    fn draw_result(&self) -> bool {
        let cyc_pos = (self.cycle as i32 - 1) % 40;
        ((cyc_pos - 1)..=(cyc_pos + 1)).contains(&self.register)
    }
}

fn part1() {
    let instructions = read_input_lines_as::<Instruction>(10);
    let state = State::initial();
    let result: i32 = [
        state.state_during_cycle(&instructions, 20),
        state.state_during_cycle(&instructions, 60),
        state.state_during_cycle(&instructions, 100),
        state.state_during_cycle(&instructions, 140),
        state.state_during_cycle(&instructions, 180),
        state.state_during_cycle(&instructions, 220),
    ]
    .iter()
    .map(|it| it.0.signal_strength())
    .sum();
    println!("Part 1: {result}");
}

fn render(draw_results: &[bool]) {
    for row in draw_results.chunks(40) {
        println!(
            "{}",
            row.iter().map(|it| if *it { '█' } else { ' ' }).join("")
        )
    }
}

fn part2() {
    let instructions = read_input_lines_as::<Instruction>(10);
    let mut draw_results = vec![];
    let mut i: u32 = 1;
    loop {
        let (s, res) = State::initial().state_during_cycle(&instructions, i);
        draw_results.push(s.draw_result());
        if matches!(res, EvalResult::Finished) {
            break;
        }
        i += 1;
    }
    println!("Part 2:");
    render(&draw_results);
}

fn main() {
    part1();
    part2();
}
