use adventofcode2022::read_input_lines_as;
use regex::Regex;
use std::convert::Infallible;
use std::fmt::Formatter;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Cost {
    ore: i64,
    clay: i64,
    obsidian: i64,
}

#[derive(Clone, Copy, Debug)]
struct Blueprint {
    id: i64,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

impl Blueprint {
    fn quality_level(self, allowed_time: i64) -> i64 {
        self.id * self.optimize(allowed_time)
    }
    fn optimize(self, allowed_time: i64) -> i64 {
        optimize_from(
            State {
                minutes_elapsed: 0,
                stocks: ResourceStockpile::default(),
                workers: RobotWorkforce {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                blueprint: self,
            },
            allowed_time,
        )
    }
}

fn optimize_from(state: State, allowed_time: i64) -> i64 {
    assert!(state.minutes_elapsed <= allowed_time);
    if state.minutes_elapsed == allowed_time {
        return state.stocks.geode;
    }
    let mut allowed_moves = Move::allowed_for_state(&state);
    if allowed_moves.contains(&Move::BuildGeode) {
        allowed_moves = vec![Move::BuildGeode];
    } else if state.stocks.obsidian >= state.blueprint.geode_robot.obsidian - state.workers.obsidian
        && state.stocks.ore + state.workers.ore - state.blueprint.obsidian_robot.ore
            < state.blueprint.geode_robot.ore
    {
        allowed_moves = vec![Move::Wait];
    } else if allowed_moves.contains(&Move::BuildObsidian) {
        allowed_moves = vec![Move::BuildObsidian];
    } else if state.stocks.clay >= state.blueprint.obsidian_robot.clay - state.workers.clay
        && state.stocks.ore + state.workers.ore - state.blueprint.clay_robot.ore
            < state.blueprint.obsidian_robot.ore
    {
        allowed_moves = vec![Move::Wait];
    } else if state.stocks.ore >= state.blueprint.clay_robot.ore
        && state.stocks.ore >= state.blueprint.ore_robot.ore
    {
        allowed_moves = vec![Move::BuildClay, Move::BuildOre];
    }

    allowed_moves
        .into_iter()
        .map(|it| optimize_from(state.apply(it), allowed_time))
        .max()
        .unwrap()
}

#[derive(Clone, Copy, Debug, Default)]
struct ResourceStockpile {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl ResourceStockpile {
    fn covers_cost(&self, cost: Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }
}

impl Sub<Cost> for ResourceStockpile {
    type Output = Self;

    fn sub(self, rhs: Cost) -> Self::Output {
        ResourceStockpile {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RobotWorkforce {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}

impl RobotWorkforce {
    fn mine(&self, stocks: ResourceStockpile) -> ResourceStockpile {
        ResourceStockpile {
            ore: stocks.ore + self.ore,
            clay: stocks.clay + self.clay,
            obsidian: stocks.obsidian + self.obsidian,
            geode: stocks.geode + self.geode,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    workers: RobotWorkforce,
    stocks: ResourceStockpile,
    blueprint: Blueprint,
    minutes_elapsed: i64,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "At {}, stocks: {:?}, workers: {:?}",
            self.minutes_elapsed, self.stocks, self.workers
        )
    }
}

impl State {
    fn apply(self, mv: Move) -> State {
        use Move::*;
        match mv {
            Wait => State {
                minutes_elapsed: self.minutes_elapsed + 1,
                stocks: self.workers.mine(self.stocks),
                ..self
            },
            BuildOre => State {
                minutes_elapsed: self.minutes_elapsed + 1,
                stocks: self.workers.mine(self.stocks) - self.blueprint.ore_robot,
                workers: RobotWorkforce {
                    ore: self.workers.ore + 1,
                    ..self.workers
                },
                ..self
            },
            BuildClay => State {
                minutes_elapsed: self.minutes_elapsed + 1,
                stocks: self.workers.mine(self.stocks) - self.blueprint.clay_robot,
                workers: RobotWorkforce {
                    clay: self.workers.clay + 1,
                    ..self.workers
                },
                ..self
            },
            BuildObsidian => State {
                minutes_elapsed: self.minutes_elapsed + 1,
                stocks: self.workers.mine(self.stocks) - self.blueprint.obsidian_robot,
                workers: RobotWorkforce {
                    obsidian: self.workers.obsidian + 1,
                    ..self.workers
                },
                ..self
            },
            BuildGeode => State {
                minutes_elapsed: self.minutes_elapsed + 1,
                stocks: self.workers.mine(self.stocks) - self.blueprint.geode_robot,
                workers: RobotWorkforce {
                    geode: self.workers.geode + 1,
                    ..self.workers
                },
                ..self
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Move {
    Wait,
    BuildOre,
    BuildClay,
    BuildObsidian,
    BuildGeode,
}

impl Move {
    fn allowed_for_state(state: &State) -> Vec<Move> {
        use Move::*;
        let mut allowed = vec![Wait];
        if state.stocks.covers_cost(state.blueprint.ore_robot) {
            allowed.push(BuildOre);
        }
        if state.stocks.covers_cost(state.blueprint.clay_robot) {
            allowed.push(BuildClay);
        }
        if state.stocks.covers_cost(state.blueprint.obsidian_robot) {
            allowed.push(BuildObsidian);
        }
        if state.stocks.covers_cost(state.blueprint.geode_robot) {
            allowed.push(BuildGeode);
        }

        allowed
    }
}

fn parse_line(s: &str) -> Blueprint {
    let re_s = concat!(
        r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_cost>\d+) ore. ",
        r"Each clay robot costs (?P<clay_cost>\d+) ore. ",
        r"Each obsidian robot costs (?P<obs_cost_ore>\d+) ore and (?P<obs_cost_clay>\d+) clay. ",
        r"Each geode robot costs (?P<geo_cost_ore>\d+) ore and (?P<geo_cost_obs>\d+) obsidian."
    );
    let parse_re = Regex::new(re_s).unwrap();
    let m = parse_re.captures(s).unwrap();
    Blueprint {
        id: m.name("id").unwrap().as_str().parse().unwrap(),
        ore_robot: Cost {
            ore: m.name("ore_cost").unwrap().as_str().parse().unwrap(),
            clay: 0,
            obsidian: 0,
        },
        clay_robot: Cost {
            ore: m.name("clay_cost").unwrap().as_str().parse().unwrap(),
            clay: 0,
            obsidian: 0,
        },
        obsidian_robot: Cost {
            ore: m.name("obs_cost_ore").unwrap().as_str().parse().unwrap(),
            clay: m.name("obs_cost_clay").unwrap().as_str().parse().unwrap(),
            obsidian: 0,
        },
        geode_robot: Cost {
            ore: m.name("geo_cost_ore").unwrap().as_str().parse().unwrap(),
            clay: 0,
            obsidian: m.name("geo_cost_obs").unwrap().as_str().parse().unwrap(),
        },
    }
}

impl FromStr for Blueprint {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_line(s))
    }
}

fn part1() {
    let inputs: Vec<Blueprint> = read_input_lines_as(19);
    let result: i64 = inputs
        .into_iter()
        .map(|it| {
            let q = it.quality_level(24);
            println!("Blueprint {}: {}", it.id, q);
            q
        })
        .sum();
    println!("Part 1: {result}");
}

fn part2() {
    let inputs: Vec<Blueprint> = read_input_lines_as(19);
    let result: i64 = inputs
        .into_iter()
        .take(3)
        .map(|it| {
            let q = it.optimize(32);
            println!("Blueprint {}: {}", it.id, q);
            q
        })
        .product();
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
