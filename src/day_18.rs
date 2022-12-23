use crate::data::Data;
use crate::nom_parser::*;
use crate::xy::Xyz;
use crate::DayOutput;
use ndarray::Array3;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum UnitCube {
    Air,
    Lava,
    Steam,
}

pub fn solve(data: &Data) -> DayOutput {
    let cubes = lines(parse_unit_cube).consume_all(data.bytes());

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for cube in &cubes {
        max_x = max_x.max(cube.x);
        max_y = max_y.max(cube.y);
        max_z = max_z.max(cube.z);
    }

    let mut droplet = Array3::from_elem(
        (max_x as usize + 1, max_y as usize + 1, max_z as usize + 1),
        UnitCube::Air,
    );
    for &cube in &cubes {
        let Xyz { x, y, z } = cube;
        droplet[[x as usize, y as usize, z as usize]] = UnitCube::Lava;
    }

    let discount = count_pairs(&droplet, |a, b| {
        use UnitCube::*;
        match (a, b) {
            (Lava, Lava) => 2,
            _ => 0,
        }
    });
    let part_1 = 6 * cubes.len() as i64 - discount;

    flood_fill_steam(&mut droplet, [0, 0, 0]);
    let discount = count_pairs(&droplet, |a, b| {
        use UnitCube::*;
        match (a, b) {
            (Lava, Lava) => 2,
            (Air, Lava) | (Lava, Air) => 1,
            _ => 0,
        }
    });
    let part_2 = 6 * cubes.len() as i64 - discount;

    (part_1, part_2).into()
}

fn parse_unit_cube(input: &[u8]) -> PResult<Xyz> {
    map(
        tuple((nom_i32, tag(b","), nom_i32, tag(b","), nom_i32)),
        |(x, _, y, _, z)| Xyz::new(x, y, z),
    )(input)
}

fn count_pairs(droplet: &Array3<UnitCube>, count: impl Fn(UnitCube, UnitCube) -> i64) -> i64 {
    let mut counter = 0;

    for window in droplet.windows((2, 1, 1)) {
        counter += count(window[[0, 0, 0]], window[[1, 0, 0]]);
    }
    for window in droplet.windows((1, 2, 1)) {
        counter += count(window[[0, 0, 0]], window[[0, 1, 0]]);
    }
    for window in droplet.windows((1, 1, 2)) {
        counter += count(window[[0, 0, 0]], window[[0, 0, 1]]);
    }

    counter
}

fn flood_fill_steam(droplet: &mut Array3<UnitCube>, start: [usize; 3]) {
    let queue = &mut VecDeque::new();
    let droplet_shape = droplet.raw_dim();

    let mut visit = |queue: &mut VecDeque<[usize; 3]>, pos: [usize; 3]| {
        if droplet[pos] == UnitCube::Air {
            droplet[pos] = UnitCube::Steam;
            queue.push_back(pos);
        }
    };

    visit(queue, start);

    while let Some(next) = queue.pop_front() {
        if next[0] > 0 {
            visit(queue, [next[0] - 1, next[1], next[2]]);
        }
        if next[1] > 0 {
            visit(queue, [next[0], next[1] - 1, next[2]]);
        }
        if next[2] > 0 {
            visit(queue, [next[0], next[1], next[2] - 1]);
        }

        if next[0] < droplet_shape[0] - 1 {
            visit(queue, [next[0] + 1, next[1], next[2]]);
        }
        if next[1] < droplet_shape[1] - 1 {
            visit(queue, [next[0], next[1] + 1, next[2]]);
        }
        if next[2] < droplet_shape[2] - 1 {
            visit(queue, [next[0], next[1], next[2] + 1]);
        }
    }
}
