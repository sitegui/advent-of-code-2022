use crate::data::Data;
use crate::nom_parser::*;
use crate::DayOutput;
use derive_more::{Add, Sub, Sum};

#[derive(Debug, Copy, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Copy, Clone, Add, Sum, Sub)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
    max_ore_robots: i32,
    max_clay_robots: i32,
    max_obsidian_robots: i32,
}

pub fn solve(data: &Data) -> DayOutput {
    let blueprints = lines(parse_blueprint).consume_all(data.bytes());

    let mut part_1 = 0;
    let initial_robot_counts = Resources {
        ore: 1,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    for blueprint in &blueprints {
        let geodes = search(blueprint, Resources::ZERO, initial_robot_counts, 24, &mut 0);

        part_1 += blueprint.id * geodes;
    }

    let mut part_2 = 1;
    for blueprint in blueprints.iter().take(3) {
        let geodes = search(blueprint, Resources::ZERO, initial_robot_counts, 32, &mut 0);

        part_2 *= geodes as i64;
    }

    (part_1 as i64, part_2).into()
}

fn parse_blueprint(input: &[u8]) -> PResult<Blueprint> {
    map(
        tuple((
            tag(b"Blueprint "),
            nom_i32,
            tag(b": "),
            parse_robot(b"ore"),
            tag(b" "),
            parse_robot(b"clay"),
            tag(b" "),
            parse_robot(b"obsidian"),
            tag(b" "),
            parse_robot(b"geode"),
        )),
        |(
            _,
            id,
            _,
            ore_robot_cost,
            _,
            clay_robot_cost,
            _,
            obsidian_robot_cost,
            _,
            geode_robot_cost,
        )| {
            Blueprint::new(
                id,
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            )
        },
    )(input)
}

fn parse_robot<'a>(resource_name: &[u8]) -> impl PParser<'a, Resources> + '_ {
    move |input| {
        map(
            tuple((
                tag(b"Each "),
                tag(resource_name),
                tag(b" robot costs "),
                parse_costs,
                tag(b"."),
            )),
            |(_, _, _, cost, _)| cost,
        )(input)
    }
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

fn search(
    blueprint: &Blueprint,
    resources: Resources,
    robot_counts: Resources,
    remaining_time: i32,
    best_geodes_so_far: &mut i32,
) -> i32 {
    if remaining_time == 0 {
        if resources.geode > *best_geodes_so_far {
            *best_geodes_so_far = resources.geode;
        }
        return resources.geode;
    }

    // Bound branches provably will not be better than the best so far
    let triangle_number = remaining_time * (remaining_time + 1) / 2;
    let geodes_maximum_bound =
        resources.geode + robot_counts.geode * remaining_time + triangle_number;
    if geodes_maximum_bound < *best_geodes_so_far {
        return 0;
    }

    // Geode is the final goal: we should build it everytime it's possible to do so
    if let Some(resources) = resources.paying(blueprint.geode_robot_cost) {
        return search(
            blueprint,
            resources + robot_counts,
            robot_counts.adding(Resource::Geode),
            remaining_time - 1,
            best_geodes_so_far,
        );
    }

    let mut best_geodes = 0;

    // Explode the consequence of building each possible robot now
    let mut can_pay_all = true;
    if let Some(resources) = resources.paying(blueprint.obsidian_robot_cost) {
        if robot_counts.obsidian < blueprint.max_obsidian_robots {
            best_geodes = best_geodes.max(search(
                blueprint,
                resources + robot_counts,
                robot_counts.adding(Resource::Obsidian),
                remaining_time - 1,
                best_geodes_so_far,
            ));
        }
    } else {
        can_pay_all = false;
    }
    if let Some(resources) = resources.paying(blueprint.clay_robot_cost) {
        if robot_counts.clay < blueprint.max_clay_robots {
            best_geodes = best_geodes.max(search(
                blueprint,
                resources + robot_counts,
                robot_counts.adding(Resource::Clay),
                remaining_time - 1,
                best_geodes_so_far,
            ));
        }
    } else {
        can_pay_all = false;
    }
    if let Some(resources) = resources.paying(blueprint.ore_robot_cost) {
        if robot_counts.ore < blueprint.max_ore_robots {
            best_geodes = best_geodes.max(search(
                blueprint,
                resources + robot_counts,
                robot_counts.adding(Resource::Ore),
                remaining_time - 1,
                best_geodes_so_far,
            ));
        }
    } else {
        can_pay_all = false;
    }

    // Explode the consequence of not building anything now
    if !can_pay_all {
        best_geodes = best_geodes.max(search(
            blueprint,
            resources + robot_counts,
            robot_counts,
            remaining_time - 1,
            best_geodes_so_far,
        ));
    }

    best_geodes
}

impl Blueprint {
    pub fn new(
        id: i32,
        ore_robot_cost: Resources,
        clay_robot_cost: Resources,
        obsidian_robot_cost: Resources,
        geode_robot_cost: Resources,
    ) -> Self {
        let costs = [
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        ];

        Self {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
            max_ore_robots: costs.iter().map(|cost| cost.ore).max().unwrap(),
            max_clay_robots: costs.iter().map(|cost| cost.clay).max().unwrap(),
            max_obsidian_robots: costs.iter().map(|cost| cost.obsidian).max().unwrap(),
        }
    }
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

    fn paying(self, cost: Self) -> Option<Self> {
        if self.ore >= cost.ore
            && self.clay >= cost.clay
            && self.obsidian >= cost.obsidian
            && self.geode >= cost.geode
        {
            Some(self - cost)
        } else {
            None
        }
    }

    fn adding(mut self, resource: Resource) -> Self {
        match resource {
            Resource::Ore => {
                self.ore += 1;
            }
            Resource::Clay => {
                self.clay += 1;
            }
            Resource::Obsidian => {
                self.obsidian += 1;
            }
            Resource::Geode => {
                self.geode += 1;
            }
        }
        self
    }
}
