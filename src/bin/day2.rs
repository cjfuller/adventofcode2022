use std::convert::Infallible;
use std::str::FromStr;

use adventofcode2022::read_input_lines_as;

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown outcome code {s}"),
        })
    }
}

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown move {s}"),
        })
    }
}

impl RPS {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn opponent_outcome(&self, outcome: Outcome) -> RPS {
        use Outcome::*;
        use RPS::*;
        match (self, outcome) {
            (_, Draw) => *self,
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
        }
    }
}

#[derive(Clone, Copy)]
struct Round {
    opponent: RPS,
    me: RPS,
}

impl FromStr for Round {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        Ok(Round {
            opponent: parts[0].parse().unwrap(),
            me: parts[1].parse().unwrap(),
        })
    }
}

impl Round {
    fn score(&self) -> i32 {
        use RPS::*;
        self.me.score()
            + match (self.opponent, self.me) {
                (Rock, Paper) => 6,
                (Rock, Scissors) => 0,
                (Rock, Rock) => 3,
                (Paper, Rock) => 0,
                (Paper, Scissors) => 6,
                (Paper, Paper) => 3,
                (Scissors, Paper) => 0,
                (Scissors, Scissors) => 3,
                (Scissors, Rock) => 6,
            }
    }
}

struct OutcomeRound {
    opponent: RPS,
    me: Outcome,
}

impl FromStr for OutcomeRound {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        Ok(OutcomeRound {
            opponent: parts[0].parse().unwrap(),
            me: parts[1].parse().unwrap(),
        })
    }
}

impl OutcomeRound {
    fn score(&self) -> i32 {
        Round {
            opponent: self.opponent,
            me: self.opponent.opponent_outcome(self.me),
        }
        .score()
    }
}

fn part1() {
    let rounds = read_input_lines_as::<Round>(2);
    let total: i32 = rounds.iter().map(|it| it.score()).sum();
    println!("Part 1: {total}")
}

fn part2() {
    let rounds = read_input_lines_as::<OutcomeRound>(2);
    let total: i32 = rounds.iter().map(|it| it.score()).sum();
    println!("Part 2: {total}")
}

fn main() {
    part1();
    part2();
}
