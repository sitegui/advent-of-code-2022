use crate::data::Data;
use crate::xy::Xy;
use crate::DayOutput;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const NORTH: Xy = Xy::new(0, -1);
const SOUTH: Xy = Xy::new(0, 1);
const EAST: Xy = Xy::new(1, 0);
const WEST: Xy = Xy::new(-1, 0);
const NORTHEAST: Xy = Xy::new(1, -1);
const NORTHWEST: Xy = Xy::new(-1, -1);
const SOUTHEAST: Xy = Xy::new(1, 1);
const SOUTHWEST: Xy = Xy::new(-1, 1);
const ALL_NEIGHBORS: [Xy; 8] = [
    NORTH, SOUTH, EAST, WEST, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST,
];

#[derive(Debug, Copy, Clone)]
struct MovementCheck {
    check: [Xy; 3],
    movement: Xy,
}

const MOVES: [MovementCheck; 4] = [
    MovementCheck {
        check: [NORTH, NORTHEAST, NORTHWEST],
        movement: NORTH,
    },
    MovementCheck {
        check: [SOUTH, SOUTHEAST, SOUTHWEST],
        movement: SOUTH,
    },
    MovementCheck {
        check: [WEST, NORTHWEST, SOUTHWEST],
        movement: WEST,
    },
    MovementCheck {
        check: [EAST, NORTHEAST, SOUTHEAST],
        movement: EAST,
    },
];

pub fn solve(data: &Data) -> DayOutput {
    let mut elves: HashSet<_> = data
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, c)| match c {
                b'#' => Some(Xy::new(x as i32, y as i32)),
                _ => None,
            })
        })
        .collect();

    let mut part_1 = None;
    let mut part_2 = None;
    for i in 0.. {
        let changed = do_round(&mut elves, i);
        if i == 9 {
            let (min, max) = bounding_box(&elves);
            let total_area = (max.x - min.x + 1) * (max.y - min.y + 1);
            part_1 = Some(total_area as i64 - elves.len() as i64);
        }

        if !changed {
            part_2 = Some(i as i64 + 1);
            break;
        }
    }

    DayOutput::Int(part_1.unwrap(), part_2.unwrap())
}

fn do_round(elves: &mut HashSet<Xy>, round: usize) -> bool {
    // Decide where each elf want to move to
    let mut desires = HashMap::with_capacity(elves.len());
    let mut elves_by_desire = HashMap::with_capacity(elves.len());
    for &elf in &*elves {
        let desire = find_target(elves, round, elf);
        desires.insert(elf, desire);
        *elves_by_desire.entry(desire).or_insert(0) += 1;
    }

    // Apply the movements
    let mut new_elves = HashSet::with_capacity(elves.len());
    for (elf, desire) in desires {
        let new_elf = if elves_by_desire[&desire] == 1 {
            desire
        } else {
            elf
        };

        new_elves.insert(new_elf);
    }

    let changed = new_elves != *elves;

    *elves = new_elves;

    changed
}

fn find_target(elves: &HashSet<Xy>, round: usize, elf: Xy) -> Xy {
    if !has_elf(elves, elf, ALL_NEIGHBORS) {
        return elf;
    }

    for i in 0..MOVES.len() {
        let movement = MOVES[(i + round) % MOVES.len()];
        if !has_elf(elves, elf, movement.check) {
            return elf + movement.movement;
        }
    }

    elf
}

fn has_elf<const N: usize>(elves: &HashSet<Xy>, elf: Xy, neighbors: [Xy; N]) -> bool {
    for neighbor in neighbors {
        if elves.contains(&(elf + neighbor)) {
            return true;
        }
    }

    false
}

fn bounding_box(elves: &HashSet<Xy>) -> (Xy, Xy) {
    let (min_x, max_x) = elves
        .iter()
        .map(|elf| elf.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = elves
        .iter()
        .map(|elf| elf.y)
        .minmax()
        .into_option()
        .unwrap();

    (Xy::new(min_x, min_y), Xy::new(max_x, max_y))
}

#[allow(unused)]
fn format_elves(elves: &HashSet<Xy>) -> String {
    let mut result = String::new();
    let (Xy { x: min_x, y: min_y }, Xy { x: max_x, y: max_y }) = bounding_box(elves);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if elves.contains(&Xy::new(x, y)) {
                '#'
            } else {
                '.'
            };
            result.push(c);
        }
        result.push('\n');
    }

    result
}
