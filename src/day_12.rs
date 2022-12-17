use crate::data::Data;
use crate::DayOutput;
use itertools::Itertools;
use pathfinding::prelude::{astar, bfs};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Area {
    i: i16,
    j: i16,
    height: i16,
}

pub fn solve(data: &Data) -> DayOutput {
    let mut start = None;
    let mut end = None;

    let height_map = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &c)| {
                    let c = match c {
                        b'S' => {
                            start = Some((i, j));
                            b'a'
                        }
                        b'E' => {
                            end = Some((i, j));
                            b'z'
                        }
                        c => c,
                    };

                    let height = c - b'a';

                    Area {
                        i: i as i16,
                        j: j as i16,
                        height: height as i16,
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let start = start.unwrap();
    let start = height_map[start.0][start.1];
    let end = end.unwrap();
    let end = height_map[end.0][end.1];

    let path = astar(
        &start,
        |base| {
            let mut neighbors = Vec::with_capacity(4);
            maybe_visit_part_1(&height_map, &mut neighbors, base, 1, 0);
            maybe_visit_part_1(&height_map, &mut neighbors, base, -1, 0);
            maybe_visit_part_1(&height_map, &mut neighbors, base, 0, 1);
            maybe_visit_part_1(&height_map, &mut neighbors, base, 0, -1);
            neighbors
        },
        |base| {
            let di = (base.i - end.i).abs();
            let dj = (base.j - end.j).abs();
            let dh = (base.height - end.height).abs();
            di + dj + dh
        },
        |base| base == &end,
    )
    .unwrap();
    let part_1 = path.0.len() as i64 - 1;

    let path = bfs(
        &end,
        |base| {
            let mut neighbors = Vec::with_capacity(4);
            maybe_visit_part_2(&height_map, &mut neighbors, base, 1, 0);
            maybe_visit_part_2(&height_map, &mut neighbors, base, -1, 0);
            maybe_visit_part_2(&height_map, &mut neighbors, base, 0, 1);
            maybe_visit_part_2(&height_map, &mut neighbors, base, 0, -1);
            neighbors
        },
        |base| base.height == 0,
    )
    .unwrap();
    let part_2 = path.len() as i64 - 1;

    (part_1, part_2).into()
}

fn maybe_visit_part_1(
    height_map: &[Vec<Area>],
    neighbors: &mut Vec<(Area, i16)>,
    base: &Area,
    di: i16,
    dj: i16,
) {
    let new_i = base.i + di;
    let new_j = base.j + dj;

    if new_i < 0 || new_i >= height_map.len() as i16 {
        return;
    }

    if new_j < 0 || new_j >= height_map[0].len() as i16 {
        return;
    }

    let neighbor = height_map[new_i as usize][new_j as usize];
    if neighbor.height > base.height + 1 {
        return;
    }

    neighbors.push((neighbor, 1));
}

fn maybe_visit_part_2(
    height_map: &[Vec<Area>],
    neighbors: &mut Vec<Area>,
    base: &Area,
    di: i16,
    dj: i16,
) {
    let new_i = base.i + di;
    let new_j = base.j + dj;

    if new_i < 0 || new_i >= height_map.len() as i16 {
        return;
    }

    if new_j < 0 || new_j >= height_map[0].len() as i16 {
        return;
    }

    let neighbor = height_map[new_i as usize][new_j as usize];
    if base.height > neighbor.height + 1 {
        return;
    }

    neighbors.push(neighbor);
}
