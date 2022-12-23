use crate::data::Data;
use crate::nom_parser::*;
use crate::xy::Xy;
use crate::DayOutput;
use itertools::Itertools;
use ndarray::{s, Array2, ArrayView2, Axis, Ix2, SliceArg, Zip};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

// The `x` axis goes from left to right
// The `y` axis goes from top to bottom
const WIDTH: i64 = 7;
const MAX_HEIGHT: i64 = 4000;

// Start position of the bottom left of the shape
const START_X: i64 = 2;
const START_Y_PADDING: i64 = 3;
const CYCLE_DEPTH: usize = 50;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Air,
    Rock,
}

type Shape = Array2<Tile>;

#[derive(Debug, Copy, Clone)]
struct FallingShape<'a> {
    bottom_left: Xy<i64>,
    shape: &'a Shape,
}

#[derive(Debug)]
struct Chamber {
    highest_rock: i64,
    /// `tiles[x][y]`
    tiles: Array2<Tile>,
}

pub fn solve(data: &Data) -> DayOutput {
    let char_to_jet = |c| match c {
        '>' => 1i64,
        '<' => -1i64,
        _ => unreachable!(),
    };
    let air_jets = terminated(
        many0(map(one_of("><".as_bytes()), char_to_jet)),
        line_ending,
    )
    .consume_all(data.bytes());

    let cycle_params = detect_cycle(&shapes(), &air_jets);

    let part_1 = cycle_params.highest_rock(2022);
    let part_2 = cycle_params.highest_rock(1_000_000_000_000);

    (part_1, part_2).into()
}

fn shapes() -> Vec<Array2<Tile>> {
    let shape_1 = Array2::from_shape_vec((4, 1), vec![Tile::Rock; 4]).unwrap();
    let shape_2 = Array2::from_shape_vec(
        (3, 3),
        vec![
            Tile::Air,
            Tile::Rock,
            Tile::Air,
            Tile::Rock,
            Tile::Rock,
            Tile::Rock,
            Tile::Air,
            Tile::Rock,
            Tile::Air,
        ],
    )
    .unwrap();
    let shape_3 = Array2::from_shape_vec(
        (3, 3),
        vec![
            Tile::Air,
            Tile::Air,
            Tile::Rock,
            Tile::Air,
            Tile::Air,
            Tile::Rock,
            Tile::Rock,
            Tile::Rock,
            Tile::Rock,
        ],
    )
    .unwrap();
    let shape_4 = Array2::from_shape_vec((1, 4), vec![Tile::Rock; 4]).unwrap();
    let shape_5 = Array2::from_shape_vec((2, 2), vec![Tile::Rock; 4]).unwrap();
    vec![shape_1, shape_2, shape_3, shape_4, shape_5]
}

#[derive(Debug, Clone)]
struct CycleParams {
    prefix_length: i64,
    cycle_length: i64,
    /// How much each cycle increases the highest rock
    cycle_rise: i64,
    /// Gives the highest rock for each step in the first execution of the cycle. Notable
    /// properties:
    /// - `highest_rock_in_cycle.len() == cycle_length as usize`
    /// - `highest_rock_in_cycle[0]` is the highest rock after `prefix_length` rocks
    highest_rock_in_cycle: Vec<i64>,
}

fn detect_cycle(shapes: &[Shape], air_jets: &[i64]) -> CycleParams {
    let mut chamber = Chamber::new(WIDTH as usize, MAX_HEIGHT as usize);
    let mut shape_index = 0;
    let mut air_jet_index = 0;
    let mut viewed_states = HashMap::new();
    let mut num_rocks = 0;
    // `max_heights[n]` gives the maximum heights after `n` rocks
    let mut max_heights = vec![];

    loop {
        max_heights.push(MAX_HEIGHT - chamber.highest_rock);

        // Detect cycle
        let state = (
            shape_index,
            air_jet_index,
            chamber.carrot(CYCLE_DEPTH).iter().copied().collect_vec(),
        );
        if let Some(prev_num_rocks) = viewed_states.insert(state, num_rocks) {
            let cycle_rise = max_heights[num_rocks] - max_heights[prev_num_rocks];
            return CycleParams {
                prefix_length: prev_num_rocks as i64,
                cycle_length: num_rocks as i64 - prev_num_rocks as i64,
                cycle_rise,
                highest_rock_in_cycle: max_heights[prev_num_rocks..num_rocks].to_vec(),
            };
        }

        let mut shape = FallingShape {
            bottom_left: Xy::new(START_X, chamber.highest_rock - START_Y_PADDING),
            shape: &shapes[shape_index],
        };
        shape_index = (shape_index + 1) % shapes.len();

        loop {
            // Lateral movement
            let air_jet = air_jets[air_jet_index];
            air_jet_index = (air_jet_index + 1) % air_jets.len();
            shape.bottom_left.x += air_jet;
            if !chamber.check_shape_position(shape) {
                shape.bottom_left.x -= air_jet;
            }

            // Vertical movement
            shape.bottom_left.y += 1;
            if !chamber.check_shape_position(shape) {
                shape.bottom_left.y -= 1;
                chamber.add_shape(shape);
                break;
            }
        }

        num_rocks += 1;
    }
}

impl Chamber {
    fn new(width: usize, height: usize) -> Self {
        let tiles = Array2::from_elem((width, height), Tile::Air);

        Chamber {
            highest_rock: height as i64,
            tiles,
        }
    }

    fn check_shape_position(&self, shape: FallingShape) -> bool {
        match self.chamber_slicing(shape) {
            None => false,
            Some(slicing) => Zip::from(self.tiles.slice(slicing))
                .and(shape.shape.view())
                .all(|&chamber_tile, &shape_tile| {
                    (chamber_tile, shape_tile) != (Tile::Rock, Tile::Rock)
                }),
        }
    }

    fn add_shape(&mut self, shape: FallingShape) {
        let slicing = self.chamber_slicing(shape).unwrap();

        Zip::from(self.tiles.slice_mut(slicing))
            .and(shape.shape.view())
            .for_each(|chamber_tile, &shape_tile| {
                if shape_tile == Tile::Rock {
                    *chamber_tile = Tile::Rock;
                }
            });

        let highest_shape_rock = shape.bottom_left.y - shape.shape.shape()[1] as i64;
        self.highest_rock = self.highest_rock.min(highest_shape_rock);
    }

    fn carrot(&self, depth: usize) -> ArrayView2<Tile> {
        let highest_rock = self.highest_rock as usize;
        let carrot_end = (highest_rock + depth).min(self.tiles.shape()[1]);
        self.tiles.slice(s![.., highest_rock..carrot_end])
    }

    fn chamber_slicing(&self, shape: FallingShape) -> Option<impl SliceArg<Ix2, OutDim = Ix2>> {
        let x_min = shape.bottom_left.x;
        let x_max = x_min + shape.shape.shape()[0] as i64;
        if x_min < 0 || x_max > self.tiles.shape()[0] as i64 {
            return None;
        }

        let y_max = shape.bottom_left.y;
        let y_min = y_max - shape.shape.shape()[1] as i64;
        if y_min < 0 || y_max > self.tiles.shape()[1] as i64 {
            return None;
        }

        Some(s![
            x_min as usize..x_max as usize,
            y_min as usize..y_max as usize
        ])
    }
}

impl CycleParams {
    fn highest_rock(&self, num_rocks: i64) -> i64 {
        assert!(num_rocks >= self.prefix_length);

        let partial_cycle = (num_rocks - self.prefix_length) % self.cycle_length;
        let num_cycles = (num_rocks - self.prefix_length) / self.cycle_length;

        self.highest_rock_in_cycle[partial_cycle as usize] + num_cycles * self.cycle_rise
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tiles.axis_iter(Axis(1)).format_with("\n", |row, f| {
                f(&format_args!("|{}|", row.iter().format("")))
            })
        )
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Air => '.',
            Tile::Rock => '#',
        })
    }
}
