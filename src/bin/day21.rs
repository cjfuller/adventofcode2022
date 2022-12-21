use adventofcode2022::read_input_lines_as;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, space1};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Multiply,
    Divide,
    Subtract,
    Eq,
}

impl Op {
    fn eval(&self, lhs: i64, rhs: i64) -> i64 {
        use Op::*;
        match self {
            Add => lhs + rhs,
            Multiply => lhs * rhs,
            Divide => lhs / rhs,
            Subtract => lhs - rhs,
            Eq => i64::from(lhs == rhs),
        }
    }
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    let (input, op) = alt((char('+'), char('-'), char('/'), char('*')))(s)?;
    let parsed = match op {
        '+' => Op::Add,
        '-' => Op::Subtract,
        '/' => Op::Divide,
        '*' => Op::Multiply,
        _ => panic!("Unknown op {op}"),
    };
    Ok((input, parsed))
}

fn parse_calc(s: &str) -> IResult<&str, Action> {
    let (input, (lhs, _, op, _, rhs)) = tuple((alpha1, space1, parse_op, space1, alpha1))(s)?;
    Ok((
        input,
        Action::Calc {
            lhs: lhs.into(),
            rhs: rhs.into(),
            op,
        },
    ))
}

fn parse_num(s: &str) -> IResult<&str, Action> {
    let (input, num) = digit1(s)?;
    Ok((input, Action::Num(num.parse().unwrap())))
}

fn parse_action(s: &str) -> IResult<&str, Action> {
    alt((parse_calc, parse_num))(s)
}

fn parse_monkey(s: &str) -> IResult<&str, Monkey> {
    let (input, (id, _, _, action)) = tuple((alpha1, tag(":"), space1, parse_action))(s)?;
    Ok((
        input,
        Monkey {
            id: id.to_string(),
            action,
        },
    ))
}

#[derive(Clone, Debug)]
enum Action {
    Num(i64),
    Calc { lhs: String, rhs: String, op: Op },
}

#[derive(Clone, Debug)]
struct Monkey {
    id: String,
    action: Action,
}

impl Monkey {
    fn eval(&self, index: &HashMap<String, Monkey>) -> i64 {
        match &self.action {
            Action::Num(n) => *n,
            Action::Calc { lhs, rhs, op } => {
                op.eval(index[lhs].eval(index), index[rhs].eval(index))
            }
        }
    }
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_monkey(s).unwrap().1)
    }
}

fn index_monkeys(monkeys: Vec<Monkey>) -> HashMap<String, Monkey> {
    let mut output = HashMap::new();
    for monkey in monkeys {
        output.insert(monkey.id.clone(), monkey);
    }
    output
}

fn part1() {
    let input = read_input_lines_as::<Monkey>(21);
    let index = index_monkeys(input);
    let result = index["root"].eval(&index);
    println!("Part 1: {result}");
}

fn find_parent(id: &str, all: &[Monkey]) -> Monkey {
    for monkey in all {
        match monkey.action {
            Action::Calc {
                ref lhs, ref rhs, ..
            } if lhs == id || rhs == id => return monkey.clone(),
            _ => (),
        }
    }
    panic!("No parent found")
}

fn build_inverted(
    from_id: &str,
    all: &[Monkey],
    index: &HashMap<String, Monkey>,
    building: &mut Vec<Monkey>,
) {
    let parent = find_parent(from_id, all);
    match parent.action {
        Action::Num(..) => panic!("Parent shouldn't have a number"),
        Action::Calc {
            lhs,
            rhs,
            op: Op::Eq,
        } => {
            if lhs == from_id {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Num(index[&rhs].eval(index)),
                })
            } else {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Num(index[&lhs].eval(index)),
                })
            }
            return;
        }
        Action::Calc {
            lhs,
            rhs,
            op: Op::Add,
        } => {
            if lhs == from_id {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: parent.id.clone(),
                        rhs: rhs,
                        op: Op::Subtract,
                    },
                })
            } else {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: parent.id.clone(),
                        rhs: lhs,
                        op: Op::Subtract,
                    },
                })
            }
        }
        Action::Calc {
            lhs,
            rhs,
            op: Op::Subtract,
        } => {
            if lhs == from_id {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: parent.id.clone(),
                        rhs: rhs,
                        op: Op::Add,
                    },
                })
            } else {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: lhs,
                        rhs: parent.id.clone(),
                        op: Op::Subtract,
                    },
                })
            }
        }
        Action::Calc {
            lhs,
            rhs,
            op: Op::Multiply,
        } => {
            if lhs == from_id {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: parent.id.clone(),
                        rhs: rhs,
                        op: Op::Divide,
                    },
                })
            } else {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: parent.id.clone(),
                        rhs: lhs,
                        op: Op::Divide,
                    },
                })
            }
        }
        Action::Calc {
            lhs,
            rhs,
            op: Op::Divide,
        } => {
            if lhs == from_id {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: parent.id.clone(),
                        rhs: rhs,
                        op: Op::Multiply,
                    },
                })
            } else {
                building.push(Monkey {
                    id: from_id.into(),
                    action: Action::Calc {
                        lhs: lhs,
                        rhs: parent.id.clone(),
                        op: Op::Divide,
                    },
                })
            }
        }
    }
    build_inverted(&parent.id, all, index, building);
}

fn part2() {
    let mut input = read_input_lines_as::<Monkey>(21);
    for monkey in input.iter_mut() {
        if &monkey.id == "root" {
            match &monkey.action {
                Action::Num(..) => panic!("root had number"),
                Action::Calc { lhs, rhs, .. } => {
                    monkey.action = Action::Calc {
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        op: Op::Eq,
                    }
                }
            }
        }
    }
    let mut index = index_monkeys(input.clone());
    let mut inverted_monkeys = vec![];
    build_inverted("humn", &input, &index, &mut inverted_monkeys);
    let new_index = index_monkeys(inverted_monkeys);
    for (k, v) in new_index {
        index.insert(k, v);
    }

    let result = index["humn"].eval(&index);
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
