use std::fmt::Display;
use std::{convert::Infallible, str::FromStr};

use adventofcode2022::read_input_lines_as;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

enum SettleResult {
    Moved { new: Point },
    Done { loc: Point },
    Abyss,
}

impl Point {
    fn iter_to(self, target: Point) -> PointIterator {
        PointIterator {
            curr: Some(self),
            from: self,
            to: target,
        }
    }

    fn settle_result(self, point: Point, grid: &Grid) -> SettleResult {
        match grid.at(point) {
            None => SettleResult::Abyss,
            Some(Location::Empty) => SettleResult::Moved { new: point },
            Some(Location::Rock) | Some(Location::Sand) => SettleResult::Done { loc: self },
        }
    }

    fn settle(self, grid: &Grid) -> SettleResult {
        let below = Point {
            x: self.x,
            y: self.y + 1,
        };
        let down_left = Point {
            x: self.x - 1,
            y: self.y + 1,
        };
        let down_right = Point {
            x: self.x + 1,
            y: self.y + 1,
        };

        for point in [below, down_left] {
            let res = self.settle_result(point, grid);
            if !matches!(res, SettleResult::Done { .. }) {
                return res;
            }
        }

        self.settle_result(down_right, grid)
    }
}

struct PointIterator {
    curr: Option<Point>,
    from: Point,
    to: Point,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.curr {
            if curr == self.to {
                self.curr = None;
                return Some(self.to);
            }
            let x_diff =
                (self.to.x - self.from.x) / std::cmp::max((self.to.x - self.from.x).abs(), 1);
            let y_diff =
                (self.to.y - self.from.y) / std::cmp::max((self.to.y - self.from.y).abs(), 1);
            let next_item = Point {
                x: curr.x + x_diff,
                y: curr.y + y_diff,
            };
            let to_yield = self.curr;
            self.curr = Some(next_item);
            to_yield
        } else {
            None
        }
    }
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        Ok(Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        })
    }
}

#[derive(Clone, Debug)]
struct PointChain(Vec<Point>);

impl FromStr for PointChain {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<Point> = s.split(" -> ").map(|it| it.parse().unwrap()).collect();
        Ok(PointChain(parts))
    }
}

#[derive(Clone, Copy, Debug)]
enum Location {
    Rock,
    Sand,
    Empty,
}

impl PointChain {
    fn draw(&self, grid: &mut [Vec<Location>], min_x: i64, min_y: i64) {
        for (start, end) in self.0.iter().tuple_windows() {
            for point in start.iter_to(*end) {
                grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = Location::Rock;
            }
        }
    }

    // Returns: (min_x, max_x), (min_y, max_y)
    fn compute_ranges(&self) -> ((i64, i64), (i64, i64)) {
        (
            (
                self.0.iter().map(|it| it.x).min().unwrap(),
                self.0.iter().map(|it| it.x).max().unwrap(),
            ),
            (
                self.0.iter().map(|it| it.y).min().unwrap(),
                self.0.iter().map(|it| it.y).max().unwrap(),
            ),
        )
    }
}

// Returns: (min_x, max_x), (min_y, max_y)
fn compute_ranges(chains: &[PointChain]) -> ((i64, i64), (i64, i64)) {
    (
        (
            chains
                .iter()
                .map(|it| it.compute_ranges().0 .0)
                .min()
                .unwrap(),
            chains
                .iter()
                .map(|it| it.compute_ranges().0 .1)
                .max()
                .unwrap(),
        ),
        (
            chains
                .iter()
                .map(|it| it.compute_ranges().1 .0)
                .min()
                .unwrap(),
            chains
                .iter()
                .map(|it| it.compute_ranges().1 .1)
                .max()
                .unwrap(),
        ),
    )
}

struct Grid {
    locations: Vec<Vec<Location>>,
    min_x: i64,
    min_y: i64,
}

impl Grid {
    fn at(&self, loc: Point) -> Option<Location> {
        if loc.x < self.min_x
            || loc.y < self.min_y
            || loc.x >= self.locations[0].len() as i64 + self.min_x
            || loc.y >= self.locations.len() as i64 + self.min_y
        {
            None
        } else {
            Some(self.locations[(loc.y - self.min_y) as usize][(loc.x - self.min_x) as usize])
        }
    }

    fn set_at(&mut self, loc: Point, value: Location) {
        if self.at(loc).is_some() {
            self.locations[(loc.y - self.min_y) as usize][(loc.x - self.min_x) as usize] = value;
        } else {
            panic!("Out of bounds set at {loc:?}");
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.locations {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|it| match it {
                        Location::Empty => ' ',
                        Location::Rock => '█',
                        Location::Sand => 'o',
                    })
                    .join("")
            )?;
        }
        Ok(())
    }
}

fn build_start_grid(chains: &[PointChain], include_floor: bool) -> Grid {
    let ((mut min_x, mut max_x), (mut min_y, mut max_y)) = compute_ranges(chains);
    // Because the sand enters from (500, 0), we have to adjust to ensure that point is in range.
    // Also adjust x so that we have at least 502 spaces to the left and right for floor.
    min_y = std::cmp::min(0, min_y);
    max_y = std::cmp::max(0, max_y + 2);
    min_x = std::cmp::min(min_x, -2);
    max_x = std::cmp::max(max_x, 1002);

    let mut grid = vec![];
    for y in min_y..=max_y {
        grid.push(vec![]);
        for _ in min_x..=max_x {
            grid[(y - min_y) as usize].push(Location::Empty);
        }
    }

    for chain in chains {
        chain.draw(&mut grid, min_x, min_y);
    }

    if include_floor {
        let floor_chain = PointChain(vec![
            Point { x: min_x, y: max_y },
            Point { x: max_x, y: max_y },
        ]);
        floor_chain.draw(&mut grid, min_x, min_y);
    }

    Grid {
        locations: grid,
        min_x,
        min_y,
    }
}

fn part1() {
    let input = read_input_lines_as::<PointChain>(14);
    let mut grid = build_start_grid(&input, false);
    let mut count: u64 = 0;

    'outer: loop {
        let mut new_grain = Point { x: 500, y: 0 };
        loop {
            match new_grain.settle(&grid) {
                SettleResult::Moved { new } => {
                    new_grain = new;
                }
                SettleResult::Done { loc } => {
                    grid.set_at(loc, Location::Sand);
                    break;
                }
                SettleResult::Abyss => {
                    break 'outer;
                }
            }
        }

        count += 1;
    }

    println!("Part 1: {count}");
}
fn part2() {
    let input = read_input_lines_as::<PointChain>(14);
    let mut grid = build_start_grid(&input, true);
    let mut count: u64 = 0;

    loop {
        let mut new_grain = Point { x: 500, y: 0 };
        loop {
            match new_grain.settle(&grid) {
                SettleResult::Moved { new } => {
                    new_grain = new;
                }
                SettleResult::Done { loc } => {
                    grid.set_at(loc, Location::Sand);
                    break;
                }
                SettleResult::Abyss => {
                    panic!("No abyss expected for part 2");
                }
            }
        }

        count += 1;

        if matches!(grid.at(Point { x: 500, y: 0 }), Some(Location::Sand)) {
            break;
        }
    }

    println!("Part 1: {count}");
}
fn main() {
    part1();
    part2();
}
