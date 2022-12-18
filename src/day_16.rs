use crate::data::Data;
use crate::nom_parser::*;
use crate::DayOutput;
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct ParsedValve {
    name: ValveName,
    flow: i32,
    neighbors: Vec<ValveName>,
}

#[derive(Debug, Clone)]
struct Valve {
    name: ValveName,
    flow: i32,
    connected_valves: HashMap<ValveName, i32>,
}

type ValveName = [u8; 2];

pub fn solve(data: &Data) -> DayOutput {
    let all_valves = lines(parse_valve).consume_all(data.bytes());

    let mut complete_graph = DiGraphMap::new();
    for valve in &all_valves {
        complete_graph.add_node(valve.name);
    }
    for valve in &all_valves {
        for &neighbor in &valve.neighbors {
            complete_graph.add_edge(valve.name, neighbor, ());
        }
    }

    let start = [b'A', b'A'];
    let interesting_valve_names: HashSet<_> = all_valves
        .iter()
        .filter(|valve| valve.name == start || valve.flow > 0)
        .map(|valve| valve.name)
        .collect();
    let mut interesting_valves = HashMap::new();
    for valve in &all_valves {
        if interesting_valve_names.contains(&valve.name) {
            let mut connected_valves = HashMap::new();
            for (another_valve, distance) in dijkstra(&complete_graph, valve.name, None, |_| 1) {
                if another_valve != start && interesting_valve_names.contains(&another_valve) {
                    connected_valves.insert(another_valve, distance);
                }
            }
            interesting_valves.insert(
                valve.name,
                Valve {
                    name: valve.name,
                    flow: valve.flow,
                    connected_valves,
                },
            );
        }
    }

    let best = &mut None;
    search_graph(&interesting_valves, &mut HashSet::new(), start, 30, 0, best);
    let part_1 = best.unwrap() as i64;

    (part_1, 0).into()
}

fn parse_valve(input: &[u8]) -> PResult<ParsedValve> {
    map(
        tuple((
            tag(b"Valve "),
            parse_name,
            tag(b" has flow rate="),
            i32,
            parse_neighbors,
        )),
        |(_, name, _, flow, neighbors)| ParsedValve {
            name,
            flow,
            neighbors,
        },
    )(input)
}

fn parse_neighbors(input: &[u8]) -> PResult<Vec<ValveName>> {
    alt((
        preceded(
            tag(b"; tunnels lead to valves "),
            separated_list1(tag(b", "), parse_name),
        ),
        map(
            preceded(tag(b"; tunnel leads to valve "), parse_name),
            |name| vec![name],
        ),
    ))(input)
}

fn parse_name(input: &[u8]) -> PResult<ValveName> {
    map(take(2usize), |name: &[u8]| [name[0], name[1]])(input)
}

fn search_graph(
    valves: &HashMap<ValveName, Valve>,
    in_path: &mut HashSet<ValveName>,
    current: ValveName,
    remaining_time: i32,
    acc_total_flow: i32,
    best_acc_total_flow: &mut Option<i32>,
) {
    struct Candidate {
        name: ValveName,
        remaining_time: i32,
        total_flow: i32,
        acc_total_flow_upper_bound: i32,
    }

    let other_valves = valves[&current]
        .connected_valves
        .iter()
        .filter(|&(name, _)| !in_path.contains(name))
        .map(|(&name, &distance)| {
            let new_remaining_time = remaining_time - distance - 1;
            let total_flow = valves[&name].flow * new_remaining_time;
            let non_opened_valves_flow: i32 = valves
                .values()
                .filter(|valve| !in_path.contains(&valve.name) && valve.name != name)
                .map(|valve| valve.flow)
                .sum();
            let acc_total_flow_upper_bound =
                acc_total_flow + total_flow + non_opened_valves_flow * (new_remaining_time - 1);
            Candidate {
                name,
                remaining_time: new_remaining_time,
                total_flow,
                acc_total_flow_upper_bound,
            }
        })
        .filter(|candidate| candidate.remaining_time > 0)
        .sorted_by_key(|candidate| candidate.total_flow);

    if other_valves.len() == 0 {
        match best_acc_total_flow {
            None => *best_acc_total_flow = Some(acc_total_flow),
            Some(best) if *best < acc_total_flow => *best = acc_total_flow,
            _ => {}
        }
    } else {
        for next in other_valves {
            if let Some(best) = best_acc_total_flow {
                if next.acc_total_flow_upper_bound < *best {
                    continue;
                }
            }
            in_path.insert(next.name);
            search_graph(
                valves,
                in_path,
                next.name,
                next.remaining_time,
                acc_total_flow + next.total_flow,
                best_acc_total_flow,
            );
            in_path.remove(&next.name);
        }
    }
}
