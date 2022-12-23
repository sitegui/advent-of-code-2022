use crate::data::Data;
use crate::nom_parser::*;
use crate::xy::Xyz;
use crate::DayOutput;
use derive_more::{Add, Sum};
use ndarray::Array3;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Add, Sum)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Copy, Clone)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    cost: Resources,
    kind: Resource,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    robots: Vec<Robot>,
}

pub fn solve(data: &Data) -> DayOutput {}

fn parse_blueprint(input: &[u8]) -> PResult<Blueprint> {
    // Blueprint 4: Each ore robot costs 4 ore.
    // Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay.
    // Each geode robot costs 4 ore and 20 obsidian.
    map(
        tuple((
            tag(b"Blueprint "),
            nom_i32,
            tag(b": Each ore robot costs "),
            nom_i32,
            tag(b" ore. Each clay robot costs "),
            nom_i32,
            tag(b" ore. Each obsidian robot costs "),
            nom_i32,
            tag(b" ore and "),
            nom_i32,
            tag(b" clay. Each geode robot costs "),
            nom_i32,
            tag(b" ore and "),
            nom_i32,
            tag(b" obsidian."),
        )),
        || {},
    )
}

fn parse_robot_cost(input: &[u8]) -> PResult<Robot> {
    map(
        tuple((
            tag(b"Each "),
            parse_resource,
            tag(b" robot costs "),
            tag(b" "),
            tag(b"."),
        )),
        || {},
    )(input)
}

fn parse_costs(input: &[u8]) -> PResult<Resources> {
    map(separated_list1(tag(b" and "), parse_cost), |costs| {
        costs.into_iter().sum()
    })(input)
}

fn parse_cost(input: &[u8]) -> PResult<Resources> {
    map(
        tuple((nom_i32, tag(b" "), parse_resource)),
        |(amount, _, resource)| {
            let mut cost = Resources::ZERO;
            *cost.get_mut(resource) += amount;
            cost
        },
    )(input)
}

fn parse_resource(input: &[u8]) -> PResult<Resource> {
    alt((
        map(tag(b"ore"), |_| Resource::Ore),
        map(tag(b"clay"), |_| Resource::Clay),
        map(tag(b"obsidian"), |_| Resource::Obsidian),
        map(tag(b"geode"), |_| Resource::Geode),
    ))(input)
}

impl Resources {
    const ZERO: Self = Self {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    fn get_mut(&mut self, resource: Resource) -> &mut i32 {
        match resource {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geode,
        }
    }
}
