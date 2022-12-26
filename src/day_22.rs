use crate::data::Data;
use crate::nom_parser::*;
use crate::DayOutput;
use itertools::Itertools;
use ndarray::Array2;
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
    prev_i: Option<Neighbor>,
    next_i: Option<Neighbor>,
    prev_j: Option<Neighbor>,
    next_j: Option<Neighbor>,
}

#[derive(Debug, Copy, Clone)]
struct Neighbor {
    id: usize,
    right_turns: u8,
}

#[derive(Debug, Copy, Clone)]
enum Heading {
    PrevJ,
    NextJ,
    PrevI,
    NextI,
}

pub fn solve(data: &Data) -> DayOutput {
    let (map, movements) = parse.consume_all(data.bytes());
    let squares = generate_squares(&map);

    let part_1 = navigate(&squares, &movements);

    let cube_side = if data.is_example() { 4 } else { 50 };
    let mut faces = detect_faces(&map, cube_side);
    link_faces_directly(&mut faces, cube_side);
    fold_cube(&mut faces);

    eprintln!("faces = {:#?}", faces);

    DayOutput::from((part_1 as i64, 0))
}

/// Apply the navigation movements and return the "password"
fn navigate(squares: &[Square], movements: &[Movement]) -> usize {
    let mut heading = Heading::NextJ;
    let mut current_square = 0;
    for &movement in movements {
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
                        Heading::PrevJ => square.prev_j,
                        Heading::NextJ => square.next_j,
                        Heading::PrevI => square.prev_i,
                        Heading::NextI => square.next_i,
                    };

                    match next {
                        None => {
                            break;
                        }
                        Some(next) => {
                            current_square = next.id;
                            for _ in 0..next.right_turns {
                                heading = heading.turned_right();
                            }
                        }
                    }
                }
            }
        }
    }

    let original = squares[current_square].original;

    1000 * (original.0 + 1) + 4 * (original.1 + 1) + heading.to_password()
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
            prev_i: None,
            next_i: None,
            prev_j: None,
            next_j: None,
        })
        .collect_vec();

    let id_by_position: HashMap<_, _> = squares
        .iter()
        .map(|square| (square.original, square.id))
        .collect();

    let position_to_neighbor = |pos| Neighbor {
        id: id_by_position[&pos],
        right_turns: 0,
    };

    for square in &mut squares {
        square.next_i = search_neighbor(map, square.original, 1, 0).map(position_to_neighbor);
        square.prev_i = search_neighbor(map, square.original, -1, 0).map(position_to_neighbor);
        square.prev_j = search_neighbor(map, square.original, 0, -1).map(position_to_neighbor);
        square.next_j = search_neighbor(map, square.original, 0, 1).map(position_to_neighbor);
    }

    squares
}

fn search_neighbor(
    map: &Array2<Tile>,
    base: Position,
    delta_i: isize,
    delta_j: isize,
) -> Option<Position> {
    let mut pos = base;
    loop {
        pos.0 = wrapping_add(pos.0, delta_i, map.shape()[0]);
        pos.1 = wrapping_add(pos.1, delta_j, map.shape()[1]);

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

fn detect_faces(map: &Array2<Tile>, cube_side: usize) -> Vec<Square> {
    let mut faces = Vec::with_capacity(6);

    for face_i in 0..map.nrows() / cube_side {
        for face_j in 0..map.ncols() / cube_side {
            let pos = (face_i * cube_side, face_j * cube_side);
            if map[pos] != Tile::Empty {
                faces.push(Square {
                    id: faces.len(),
                    original: pos,
                    prev_i: None,
                    next_i: None,
                    prev_j: None,
                    next_j: None,
                });
            }
        }
    }

    faces
}

fn link_faces_directly(faces: &mut [Square], cube_side: usize) {
    fn link_i(faces: &mut [Square], prev: usize, next: usize) {
        faces[prev].next_i = Some(Neighbor {
            id: next,
            right_turns: 0,
        });
        faces[next].prev_i = Some(Neighbor {
            id: prev,
            right_turns: 0,
        });
    }

    fn link_j(faces: &mut [Square], prev: usize, next: usize) {
        faces[prev].next_j = Some(Neighbor {
            id: next,
            right_turns: 0,
        });
        faces[next].prev_j = Some(Neighbor {
            id: prev,
            right_turns: 0,
        });
    }

    for face_a in 0..faces.len() {
        for face_b in face_a + 1..faces.len() {
            let face_a_pos = faces[face_a].original;
            let face_b_pos = faces[face_b].original;
            let delta_i = (face_b_pos.0 as isize - face_a_pos.0 as isize) / cube_side as isize;
            let delta_j = (face_b_pos.1 as isize - face_a_pos.1 as isize) / cube_side as isize;

            match (delta_i, delta_j) {
                (1, 0) => link_i(faces, face_a, face_b),
                (-1, 0) => link_i(faces, face_b, face_a),
                (0, 1) => link_j(faces, face_a, face_b),
                (0, -1) => link_j(faces, face_a, face_b),
                _ => {}
            }
        }
    }
}

fn fold_cube(faces: &mut [Square]) {
    fn print_faces(faces: &[Square]) {
        for face in faces {
            println!(
                "Face {}: ni = {:?}, nj = {:?}, pi = {:?}, pj = {:?}",
                face.id,
                face.next_i.map(|n| n.id),
                face.next_j.map(|n| n.id),
                face.prev_i.map(|n| n.id),
                face.prev_j.map(|n| n.id)
            );
        }
    }

    for _ in 0..5 {
        for middle_face_id in 0..faces.len() {
            let middle_face = faces[middle_face_id];

            macro_rules! fold_neighbors {
                ($a:ident, $b:ident) => {
                    if let Some((neighbor_1, neighbor_2)) = middle_face.$a.zip(middle_face.$b) {
                        println!("{} {} {}", middle_face_id, stringify!($a), stringify!($b));
                        faces[neighbor_1.id].$b = Some(Neighbor {
                            id: neighbor_2.id,
                            right_turns: 3,
                        });
                        faces[neighbor_2.id].$a = Some(Neighbor {
                            id: neighbor_1.id,
                            right_turns: 1,
                        });
                        print_faces(faces);
                    }
                };
            }

            fold_neighbors!(next_i, next_j);
            fold_neighbors!(prev_j, next_i);
            fold_neighbors!(prev_i, prev_j);
            fold_neighbors!(next_j, prev_i);
        }

        if faces.iter().all(|face| {
            face.prev_i.is_some()
                && face.prev_j.is_some()
                && face.next_i.is_some()
                && face.next_j.is_some()
        }) {
            break;
        }
    }
}

impl Heading {
    fn turned_left(self) -> Self {
        match self {
            Heading::PrevJ => Heading::NextI,
            Heading::NextJ => Heading::PrevI,
            Heading::PrevI => Heading::PrevJ,
            Heading::NextI => Heading::NextJ,
        }
    }

    fn turned_right(self) -> Self {
        match self {
            Heading::PrevJ => Heading::PrevI,
            Heading::NextJ => Heading::NextI,
            Heading::PrevI => Heading::NextJ,
            Heading::NextI => Heading::PrevJ,
        }
    }

    fn to_password(self) -> usize {
        match self {
            Heading::PrevJ => 2,
            Heading::NextJ => 0,
            Heading::PrevI => 3,
            Heading::NextI => 1,
        }
    }
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
