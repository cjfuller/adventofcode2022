use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

use adventofcode2022::read_blank_line_delimited_blocks_as;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Old,
    Lit(u64),
}

impl Arg {
    fn eval(self, curr_worry: u64) -> u64 {
        match self {
            Arg::Old => curr_worry,
            Arg::Lit(n) => n,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Plus,
    Times,
}

impl Op {
    fn eval(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Plus => lhs + rhs,
            Self::Times => lhs * rhs,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    index: usize,
    starting_items: Vec<u64>,
    transform: (Op, Arg, Arg),
    test_divisor: u64,
    true_target_index: usize,
    false_target_index: usize,
}

fn parse_monkey_index(s: &str) -> IResult<&str, usize> {
    let (input, (_, _, monkey_index, _, _)) =
        tuple((tag("Monkey"), space1, digit1, char(':'), newline))(s)?;
    Ok((input, monkey_index.parse().unwrap()))
}

fn parse_items(s: &str) -> IResult<&str, Vec<u64>> {
    let (input, (_, _, _, items, _)) = tuple((
        space1,
        tag("Starting items:"),
        space1,
        separated_list1(tag(", "), digit1),
        newline,
    ))(s)?;
    let parsed_items = items
        .into_iter()
        .map(|it| it.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    Ok((input, parsed_items))
}

fn parse_operand(s: &str) -> IResult<&str, Arg> {
    let (input, arg) = alt((tag("old"), digit1))(s)?;

    Ok((
        input,
        match arg {
            "old" => Arg::Old,
            a => Arg::Lit(a.parse().unwrap()),
        },
    ))
}

fn parse_operator(s: &str) -> IResult<&str, Op> {
    let (input, op) = alt((char('+'), char('*')))(s)?;

    Ok((
        input,
        match op {
            '+' => Op::Plus,
            '*' => Op::Times,
            other => panic!("Unknown op {other}"),
        },
    ))
}

fn parse_transform(s: &str) -> IResult<&str, (Op, Arg, Arg)> {
    let (input, (_, _, _, _, _, lhs, _, op, _, rhs, _)) = tuple((
        space1,
        tag("Operation:"),
        space1,
        tag("new ="),
        space1,
        parse_operand,
        space1,
        parse_operator,
        space1,
        parse_operand,
        newline,
    ))(s)?;
    Ok((input, (op, lhs, rhs)))
}

fn parse_test_divisor(s: &str) -> IResult<&str, u64> {
    let (input, (_, _, _, num, _)) =
        tuple((space1, tag("Test: divisible by"), space1, digit1, newline))(s)?;
    Ok((input, num.parse().unwrap()))
}

fn parse_true_branch(s: &str) -> IResult<&str, usize> {
    let (input, (_, _, _, num, _)) = tuple((
        space1,
        tag("If true: throw to monkey"),
        space1,
        digit1,
        newline,
    ))(s)?;
    Ok((input, num.parse().unwrap()))
}

fn parse_false_branch(s: &str) -> IResult<&str, usize> {
    let (input, (_, _, _, num, _)) = tuple((
        space1,
        tag("If false: throw to monkey"),
        space1,
        digit1,
        opt(newline),
    ))(s)?;
    Ok((input, num.parse().unwrap()))
}

fn parse_monkey(s: &str) -> Monkey {
    let (_, (index, items, transform, divisor, tr, fls)) = tuple((
        parse_monkey_index,
        parse_items,
        parse_transform,
        parse_test_divisor,
        parse_true_branch,
        parse_false_branch,
    ))(s)
    .unwrap();
    Monkey {
        index,
        starting_items: items,
        transform,
        test_divisor: divisor,
        true_target_index: tr,
        false_target_index: fls,
    }
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_monkey(s))
    }
}

fn eval_worry_transform(curr: u64, transform: (Op, Arg, Arg)) -> u64 {
    let (op, lhs, rhs) = transform;
    op.eval(lhs.eval(curr), rhs.eval(curr))
}

fn update_inspection_count(sum: &mut HashMap<usize, usize>, additional: HashMap<usize, usize>) {
    for (monkey, count) in additional {
        sum.insert(monkey, count + sum.get(&monkey).copied().unwrap_or(0));
    }
}

fn run_one_round(
    monkeys: &mut [Monkey],
    worry_management_fn: impl Fn(u64) -> u64,
) -> HashMap<usize, usize> {
    let mut inspections = HashMap::new();
    for i in 0..monkeys.len() {
        let mut items = vec![];
        assert!(monkeys[i].index == i);
        std::mem::swap(&mut monkeys[i].starting_items, &mut items);
        inspections.insert(i, items.len());
        for item in items {
            let mut new_worry = eval_worry_transform(item, monkeys[i].transform);
            new_worry = worry_management_fn(new_worry);
            let target_index = if new_worry % monkeys[i].test_divisor == 0 {
                monkeys[i].true_target_index
            } else {
                monkeys[i].false_target_index
            };
            assert!(monkeys[target_index].index == target_index);
            monkeys[target_index].starting_items.push(new_worry);
        }
    }
    inspections
}

fn part1() {
    let mut monkeys: Vec<Monkey> = read_blank_line_delimited_blocks_as(11);
    let mut total_inspections = HashMap::new();
    for _ in 0..20 {
        let inspections = run_one_round(&mut monkeys, |worry| worry / 3);
        update_inspection_count(&mut total_inspections, inspections);
    }
    let monkey_business: usize = total_inspections.values().sorted().rev().take(2).product();
    println!("Part 1: {monkey_business}");
}

fn part2() {
    let mut monkeys: Vec<Monkey> = read_blank_line_delimited_blocks_as(11);
    let monkey_divisors: HashSet<u64> = monkeys.iter().map(|it| it.test_divisor).collect();
    let monkey_lcm: u64 = monkey_divisors.iter().product();
    let mut total_inspections = HashMap::new();
    for _ in 0..10000 {
        let inspections = run_one_round(&mut monkeys, |worry| worry % monkey_lcm);
        update_inspection_count(&mut total_inspections, inspections);
    }
    let monkey_business: usize = total_inspections.values().sorted().rev().take(2).product();
    println!("Part 2: {monkey_business}");
}

fn main() {
    part1();
    part2();
}
