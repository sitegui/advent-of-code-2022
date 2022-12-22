use crate::data::Data;
use crate::nom_parser::*;
use crate::xy::XY;
use crate::DayOutput;
use itertools::Itertools;
use ndarray::{s, Array2, Axis, Ix2, SliceArg, Zip};
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
}

type Shape = Array2<Tile>;

#[derive(Debug, Copy, Clone)]
struct FallingShape<'a> {
    bottom_left: XY<isize>,
    tiles: &'a Shape,
}

#[derive(Debug)]
struct Chamber {
    highest_rock: isize,
    tiles: Array2<Tile>,
}

pub fn solve(data: &Data) -> DayOutput {
    // The `x` axis goes from left to right
    // The `y` axis goes from top to bottom
    const WIDTH: isize = 7;
    const MAX_HEIGHT: isize = 4000;

    // Start position of the bottom left of the shape
    const START_X: isize = 2;
    const START_Y_PADDING: isize = 3;

    const ROCKS: usize = 2022;

    // chamber[x][y]
    let mut chamber = Chamber::new(WIDTH as usize, MAX_HEIGHT as usize);
    let shapes = shapes();
    let char_to_jet = |c| match c {
        '>' => 1isize,
        '<' => -1isize,
        _ => unreachable!(),
    };
    let air_jets = terminated(
        many0(map(one_of("><".as_bytes()), char_to_jet)),
        line_ending,
    )
    .consume_all(data.bytes());

    let mut incoming_shapes = shapes.iter().cycle();
    let mut incoming_air_jet = air_jets.iter().cycle();

    for _ in 0..ROCKS {
        let mut shape = FallingShape {
            bottom_left: XY::new(START_X, chamber.highest_rock - START_Y_PADDING),
            tiles: incoming_shapes.next().unwrap(),
        };

        loop {
            // Lateral movement
            let air_jet = *incoming_air_jet.next().unwrap();
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
    }

    let part_1 = MAX_HEIGHT - chamber.highest_rock;

    println!("{}", chamber);

    (part_1 as i64, 0).into()
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

impl Chamber {
    fn new(width: usize, height: usize) -> Self {
        let tiles = Array2::from_elem((width, height), Tile::Air);

        Chamber {
            highest_rock: height as isize,
            tiles,
        }
    }

    fn check_shape_position(&self, shape: FallingShape) -> bool {
        match self.chamber_slicing(shape) {
            None => false,
            Some(slicing) => Zip::from(self.tiles.slice(slicing))
                .and(shape.tiles.view())
                .all(|&chamber_tile, &shape_tile| {
                    (chamber_tile, shape_tile) != (Tile::Rock, Tile::Rock)
                }),
        }
    }

    fn add_shape(&mut self, shape: FallingShape) {
        let slicing = self.chamber_slicing(shape).unwrap();

        Zip::from(self.tiles.slice_mut(slicing))
            .and(shape.tiles.view())
            .for_each(|chamber_tile, &shape_tile| {
                if shape_tile == Tile::Rock {
                    *chamber_tile = Tile::Rock;
                }
            });

        let highest_shape_rock = shape.bottom_left.y - shape.tiles.shape()[1] as isize;
        self.highest_rock = self.highest_rock.min(highest_shape_rock);
    }

    fn chamber_slicing(&self, shape: FallingShape) -> Option<impl SliceArg<Ix2, OutDim = Ix2>> {
        let x_min = shape.bottom_left.x;
        let x_max = x_min + shape.tiles.shape()[0] as isize;
        if x_min < 0 || x_max > self.tiles.shape()[0] as isize {
            return None;
        }

        let y_max = shape.bottom_left.y;
        let y_min = y_max - shape.tiles.shape()[1] as isize;
        if y_min < 0 || y_max > self.tiles.shape()[1] as isize {
            return None;
        }

        Some(s![
            x_min as usize..x_max as usize,
            y_min as usize..y_max as usize
        ])
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
