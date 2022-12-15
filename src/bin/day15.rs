use std::collections::HashSet;
use std::convert::Infallible;
use std::ops::RangeInclusive;
use std::str::FromStr;

use adventofcode2022::read_input_lines_as;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, space1};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn tuning_frequency(&self) -> i64 {
        self.x * 4000000 + self.y
    }
}

#[derive(Clone, Copy, Debug)]
struct Sensor {
    loc: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn dist_to_closest(&self) -> i64 {
        (self.loc.x - self.closest_beacon.x).abs() + (self.loc.y - self.closest_beacon.y).abs()
    }
}

fn num(s: &str) -> IResult<&str, i64> {
    let (input, parts) = many1(alt((digit1, tag("-"))))(s)?;
    Ok((input, parts.join("").parse().unwrap()))
}

fn parse_point(s: &str) -> IResult<&str, Point> {
    let (input, (_, x, _, _, y)) = tuple((tag("x="), num, tag(", "), tag("y="), num))(s)?;
    Ok((input, Point { x, y }))
}

fn parse_sensor_loc(s: &str) -> IResult<&str, Point> {
    let (input, (_, _, loc)) = tuple((tag("Sensor at"), space1, parse_point))(s)?;
    Ok((input, loc))
}

fn parse_beacon(s: &str) -> IResult<&str, Point> {
    let (input, (_, _, loc)) = tuple((tag("closest beacon is at"), space1, parse_point))(s)?;
    Ok((input, loc))
}

fn parse_sensor(s: &str) -> IResult<&str, Sensor> {
    let (input, (sensor_loc, _, _, beacon_loc)) =
        tuple((parse_sensor_loc, char(':'), space1, parse_beacon))(s)?;
    Ok((
        input,
        Sensor {
            loc: sensor_loc,
            closest_beacon: beacon_loc,
        },
    ))
}

impl FromStr for Sensor {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_sensor(s).unwrap().1)
    }
}

struct RangeSet {
    ranges: Vec<RangeInclusive<i64>>,
}

fn try_merge(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    if a.end() < b.start() || b.end() < a.start() {
        return None;
    }
    if a.end() <= b.end() && a.start() >= b.start() {
        return Some(b.clone());
    }

    if b.end() <= a.end() && b.start() >= a.start() {
        return Some(a.clone());
    }

    if a.start() <= b.start() && a.end() <= b.end() {
        return Some(*a.start()..=*b.end());
    }

    if b.start() <= a.start() && b.end() <= a.end() {
        return Some(*b.start()..=*a.end());
    }

    unreachable!("Checked overlap possibilities exhaustively.")
}

impl RangeSet {
    fn new() -> RangeSet {
        RangeSet { ranges: vec![] }
    }

    fn insert(&mut self, item: RangeInclusive<i64>) {
        let mut next = vec![];
        for curr in &self.ranges {
            if let Some(merged) = try_merge(&item, curr) {
                next.push(merged);
            } else {
                next.push(curr.clone());
            }
        }
        if next.len() == self.ranges.len() {
            next.push(item);
        }
        self.ranges = next;
        self.merge_self();
    }

    fn merge_self(&mut self) {
        self.ranges.sort_by_key(|it| *it.start());
        let ranges = std::mem::take(&mut self.ranges);
        let mut last = Some(ranges[0].clone());
        for ((_, curr0), (ix1, curr1)) in ranges.iter().enumerate().tuple_windows() {
            if let Some(merged) = try_merge(curr0, curr1) {
                self.ranges.push(merged);
                self.ranges
                    .extend(ranges[(ix1 + 1)..ranges.len()].iter().cloned());
                return self.merge_self();
            } else {
                self.ranges.push(curr0.clone());
                last = Some(curr1.clone());
            }
        }

        if let Some(l) = last {
            self.ranges.push(l);
        }
    }

    fn size(&self) -> i64 {
        self.ranges
            .iter()
            .map(|it| *it.end() - *it.start() + 1)
            .sum()
    }
}

fn count_disallowed_in_row(row: i64, sensors: &[Sensor]) -> usize {
    let mut all_disallowed = HashSet::new();
    for sensor in sensors {
        let dist = sensor.dist_to_closest();
        let dist_to_row = (sensor.loc.y - row).abs();
        let extra_dist = dist - dist_to_row;
        if extra_dist <= 0 {
            continue;
        }
        let x_poses = (sensor.loc.x - extra_dist)..=(sensor.loc.x + extra_dist);
        all_disallowed.extend(x_poses);
    }

    for sensor in sensors {
        if sensor.closest_beacon.y == row {
            all_disallowed.remove(&sensor.closest_beacon.x);
        }
    }

    all_disallowed.len()
}

fn find_allowed_in_row(row: i64, sensors: &[Sensor]) -> Option<Point> {
    let mut all_disallowed = RangeSet::new();
    for sensor in sensors {
        let dist = sensor.dist_to_closest();
        let dist_to_row = (sensor.loc.y - row).abs();
        let extra_dist = dist - dist_to_row;
        if extra_dist <= 0 {
            continue;
        }
        let x_poses = std::cmp::max(sensor.loc.x - extra_dist, 0)
            ..=std::cmp::min(sensor.loc.x + extra_dist, 4000000);
        all_disallowed.insert(x_poses);
        if all_disallowed.size() == 4000001 {
            break;
        }
    }

    if all_disallowed.size() < 4000001 {
        assert!(all_disallowed.ranges.len() == 2);
        if *all_disallowed.ranges[0].start() > 0 {
            Some(Point { x: 0, y: row })
        } else {
            Some(Point {
                x: *all_disallowed.ranges[0].end() + 1,
                y: row,
            })
        }
    } else {
        None
    }
}

fn part1() {
    let input = read_input_lines_as::<Sensor>(15);
    let result = count_disallowed_in_row(2000000, &input);
    println!("Part 1: {:?}", result);
}

fn part2() {
    let input = read_input_lines_as::<Sensor>(15);
    for row in 0..=4000000 {
        if let Some(coord) = find_allowed_in_row(row, &input) {
            println!("Part 2: {}", coord.tuning_frequency());
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}
