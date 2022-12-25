use crate::data::Data;
use crate::nom_parser::*;
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

type Position = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Open,
    Wall,
}

#[derive(Debug, Copy, Clone)]
enum Movement {
    TurnRight,
    TurnLeft,
    Advance(usize),
}

#[derive(Debug, Copy, Clone)]
struct Square {
    id: usize,
    original: Position,
    left: Option<usize>,
    right: Option<usize>,
    up: Option<usize>,
    down: Option<usize>,
}

#[derive(Debug, Copy, Clone)]
enum Heading {
    Left,
    Right,
    Up,
    Down,
}

pub fn solve(data: &Data) -> DayOutput {
    let (map, movements) = parse.consume_all(data.bytes());
    let squares = generate_squares(&map);

    let mut heading = Heading::Right;
    let mut current_square = 0;
    for &movement in &movements {
        match movement {
            Movement::TurnRight => {
                heading = heading.turned_right();
            }
            Movement::TurnLeft => {
                heading = heading.turned_left();
            }
            Movement::Advance(num) => {
                for _ in 0..num {
                    let square = squares[current_square];
                    let next = match heading {
                        Heading::Left => square.left,
                        Heading::Right => square.right,
                        Heading::Up => square.up,
                        Heading::Down => square.down,
                    };

                    match next {
                        None => {
                            break;
                        }
                        Some(next) => {
                            current_square = next;
                        }
                    }
                }
            }
        }
    }

    DayOutput::from((0, 0))
}

fn parse(input: &[u8]) -> PResult<(Array2<Tile>, Vec<Movement>)> {
    map(
        tuple((lines(parse_tiles), many0(parse_movement), line_ending)),
        |(rows, movements, _)| {
            let largest_row = rows.iter().map(|row| row.len()).max().unwrap();
            let map = Array2::from_shape_fn((rows.len(), largest_row), |(i, j)| {
                rows[i].get(j).copied().unwrap_or(Tile::Empty)
            });
            (map, movements)
        },
    )(input)
}

fn parse_tiles(input: &[u8]) -> PResult<Vec<Tile>> {
    many0(map(one_of(" .#".as_bytes()), |c: char| match c {
        ' ' => Tile::Empty,
        '.' => Tile::Open,
        '#' => Tile::Wall,
        _ => unreachable!(),
    }))(input)
}

fn parse_movement(input: &[u8]) -> PResult<Movement> {
    alt((
        map(nom_i32, |n| Movement::Advance(n as usize)),
        map(tag(b"R"), |_| Movement::TurnRight),
        map(tag(b"L"), |_| Movement::TurnLeft),
    ))(input)
}

fn generate_squares(map: &Array2<Tile>) -> Vec<Square> {
    let mut squares = map
        .indexed_iter()
        .filter(|&(_, &tile)| tile == Tile::Open)
        .enumerate()
        .map(|(id, (original_position, _))| Square {
            id,
            original: original_position,
            left: None,
            right: None,
            up: None,
            down: None,
        })
        .collect_vec();

    let id_by_position: HashMap<_, _> = squares
        .iter()
        .map(|square| (square.original, square.id))
        .collect();

    for square in &mut squares {
        square.right = search_neighbor(map, square.original, 1, 0).map(|pos| id_by_position[&pos]);
        square.left = search_neighbor(map, square.original, -1, 0).map(|pos| id_by_position[&pos]);
        square.up = search_neighbor(map, square.original, 0, -1).map(|pos| id_by_position[&pos]);
        square.down = search_neighbor(map, square.original, 0, 1).map(|pos| id_by_position[&pos]);
    }

    squares
}

fn search_neighbor(
    map: &Array2<Tile>,
    base: Position,
    delta_x: isize,
    delta_y: isize,
) -> Option<Position> {
    let mut pos = base;
    loop {
        pos.0 = wrapping_add(pos.0, delta_x, map.shape()[0]);
        pos.1 = wrapping_add(pos.1, delta_y, map.shape()[1]);

        match map[pos] {
            Tile::Empty => {}
            Tile::Open => return Some(pos),
            Tile::Wall => return None,
        }
    }
}

fn wrapping_add(a: usize, b: isize, len: usize) -> usize {
    (a as isize + b).rem_euclid(len as isize) as usize
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Empty => ' ',
            Tile::Open => '.',
            Tile::Wall => '#',
        })
    }
}
