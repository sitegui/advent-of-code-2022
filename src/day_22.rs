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
    heading: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    PrevJ,
    NextJ,
    PrevI,
    NextI,
}

pub fn solve(data: &Data) -> DayOutput {
    let (map, movements) = parse.consume_all(data.bytes());
    let squares_part_1 = generate_squares(&map, |base, direction| {
        search_neighbor_part_1(&map, base, direction)
    });

    let part_1 = navigate_and_get_password(&squares_part_1, &movements);

    let cube_side = if data.is_example() { 4 } else { 50 };
    let mut faces = detect_faces(&map, cube_side);
    link_faces_directly(&mut faces, cube_side);
    fold_cube(&mut faces);

    let squares_part_2 = generate_squares(&map, |base, direction| {
        search_neighbor_part_2(&map, &faces, cube_side, base, direction)
    });
    let part_2 = navigate_and_get_password(&squares_part_2, &movements);

    DayOutput::from((part_1 as i64, part_2 as i64))
}

/// Apply the navigation movements and return the "password"
fn navigate_and_get_password(squares: &[Square], movements: &[Movement]) -> usize {
    let mut heading = Direction::NextJ;
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
                        Direction::PrevJ => square.prev_j,
                        Direction::NextJ => square.next_j,
                        Direction::PrevI => square.prev_i,
                        Direction::NextI => square.next_i,
                    };

                    match next {
                        None => {
                            break;
                        }
                        Some(next) => {
                            current_square = next.id;
                            heading = next.heading;
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

fn generate_squares(
    map: &Array2<Tile>,
    mut search_neighbor: impl FnMut(Position, Direction) -> Option<(Position, Direction)>,
) -> Vec<Square> {
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

    for square in &mut squares {
        for heading in [
            Direction::PrevJ,
            Direction::NextJ,
            Direction::PrevI,
            Direction::NextI,
        ] {
            *square.get_mut(heading) =
                search_neighbor(square.original, heading).map(|(pos, heading)| Neighbor {
                    id: id_by_position[&pos],
                    heading,
                });
        }
    }

    squares
}

fn search_neighbor_part_1(
    map: &Array2<Tile>,
    base: Position,
    direction: Direction,
) -> Option<(Position, Direction)> {
    let (delta_i, delta_j) = direction.deltas();

    let mut pos = base;
    loop {
        pos.0 = wrapping_add(pos.0, delta_i, map.shape()[0]);
        pos.1 = wrapping_add(pos.1, delta_j, map.shape()[1]);

        match map[pos] {
            Tile::Empty => {}
            Tile::Open => return Some((pos, direction)),
            Tile::Wall => return None,
        }
    }
}

fn search_neighbor_part_2(
    map: &Array2<Tile>,
    faces: &[Square],
    cube_side: usize,
    base: Position,
    direction: Direction,
) -> Option<(Position, Direction)> {
    let (delta_i, delta_j) = direction.deltas();
    let next_pos = (base.0 as isize + delta_i, base.1 as isize + delta_j);
    let simple_walk = if next_pos.0 >= 0 && next_pos.1 >= 0 {
        map.get((next_pos.0 as usize, next_pos.1 as usize))
            .copied()
            .unwrap_or(Tile::Empty)
    } else {
        Tile::Empty
    };

    match simple_walk {
        Tile::Open => return Some(((next_pos.0 as usize, next_pos.1 as usize), direction)),
        Tile::Wall => return None,
        Tile::Empty => {}
    }

    let face_pin = (
        base.0 / cube_side * cube_side,
        base.1 / cube_side * cube_side,
    );
    let face = faces.iter().find(|face| face.original == face_pin).unwrap();
    let neighbor = face.get(direction).unwrap();
    let neighbor_pin = faces[neighbor.id].original;

    let corner_distance = match direction {
        Direction::PrevJ => face_pin.0 + (cube_side - 1) - base.0,
        Direction::NextJ => base.0 - face_pin.0,
        Direction::PrevI => base.1 - face_pin.1,
        Direction::NextI => face_pin.1 + (cube_side - 1) - base.1,
    };
    let next_pos = match neighbor.heading {
        Direction::PrevJ => (
            neighbor_pin.0 + (cube_side - 1) - corner_distance,
            neighbor_pin.1 + (cube_side - 1),
        ),
        Direction::NextJ => (neighbor_pin.0 + corner_distance, neighbor_pin.1),
        Direction::PrevI => (
            neighbor_pin.0 + (cube_side - 1),
            neighbor_pin.1 + corner_distance,
        ),
        Direction::NextI => (
            neighbor_pin.0,
            neighbor_pin.1 + (cube_side - 1) - corner_distance,
        ),
    };

    match map[next_pos] {
        Tile::Open => Some((next_pos, neighbor.heading)),
        Tile::Wall => None,
        _ => unreachable!(),
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
            heading: Direction::NextI,
        });
        faces[next].prev_i = Some(Neighbor {
            id: prev,
            heading: Direction::PrevI,
        });
    }

    fn link_j(faces: &mut [Square], prev: usize, next: usize) {
        faces[prev].next_j = Some(Neighbor {
            id: next,
            heading: Direction::NextJ,
        });
        faces[next].prev_j = Some(Neighbor {
            id: prev,
            heading: Direction::PrevJ,
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
    for _ in 0..5 {
        for middle_face_id in 0..faces.len() {
            let middle_face = faces[middle_face_id];

            for (border_1, border_2) in [
                (Direction::NextI, Direction::NextJ),
                (Direction::PrevJ, Direction::NextI),
                (Direction::PrevI, Direction::PrevJ),
                (Direction::NextJ, Direction::PrevI),
            ] {
                let neighbor_1 = middle_face.get(border_1);
                let neighbor_2 = middle_face.get(border_2);
                if let Some((neighbor_1, neighbor_2)) = neighbor_1.zip(neighbor_2) {
                    let border_1 = faces[neighbor_1.id]
                        .find_border(middle_face_id)
                        .unwrap()
                        .turned_right();
                    let border_2 = faces[neighbor_2.id]
                        .find_border(middle_face_id)
                        .unwrap()
                        .turned_left();

                    *faces[neighbor_1.id].get_mut(border_1) = Some(Neighbor {
                        id: neighbor_2.id,
                        heading: border_2.flipped(),
                    });
                    *faces[neighbor_2.id].get_mut(border_2) = Some(Neighbor {
                        id: neighbor_1.id,
                        heading: border_1.flipped(),
                    });
                }
            }
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

impl Square {
    fn get(&self, border: Direction) -> Option<Neighbor> {
        match border {
            Direction::PrevJ => self.prev_j,
            Direction::NextJ => self.next_j,
            Direction::PrevI => self.prev_i,
            Direction::NextI => self.next_i,
        }
    }

    fn get_mut(&mut self, border: Direction) -> &mut Option<Neighbor> {
        match border {
            Direction::PrevJ => &mut self.prev_j,
            Direction::NextJ => &mut self.next_j,
            Direction::PrevI => &mut self.prev_i,
            Direction::NextI => &mut self.next_i,
        }
    }

    fn find_border(&self, neighbor: usize) -> Option<Direction> {
        match (self.prev_j, self.next_j, self.prev_i, self.next_i) {
            (Some(Neighbor { id, .. }), _, _, _) if id == neighbor => Some(Direction::PrevJ),
            (_, Some(Neighbor { id, .. }), _, _) if id == neighbor => Some(Direction::NextJ),
            (_, _, Some(Neighbor { id, .. }), _) if id == neighbor => Some(Direction::PrevI),
            (_, _, _, Some(Neighbor { id, .. })) if id == neighbor => Some(Direction::NextI),
            _ => None,
        }
    }
}

impl Direction {
    fn turned_left(self) -> Self {
        match self {
            Direction::PrevJ => Direction::NextI,
            Direction::NextJ => Direction::PrevI,
            Direction::PrevI => Direction::PrevJ,
            Direction::NextI => Direction::NextJ,
        }
    }

    fn turned_right(self) -> Self {
        match self {
            Direction::PrevJ => Direction::PrevI,
            Direction::NextJ => Direction::NextI,
            Direction::PrevI => Direction::NextJ,
            Direction::NextI => Direction::PrevJ,
        }
    }

    fn flipped(self) -> Self {
        match self {
            Direction::PrevJ => Direction::NextJ,
            Direction::NextJ => Direction::PrevJ,
            Direction::PrevI => Direction::NextI,
            Direction::NextI => Direction::PrevI,
        }
    }

    fn to_password(self) -> usize {
        match self {
            Direction::PrevJ => 2,
            Direction::NextJ => 0,
            Direction::PrevI => 3,
            Direction::NextI => 1,
        }
    }

    fn deltas(self) -> (isize, isize) {
        match self {
            Direction::PrevJ => (0, -1),
            Direction::NextJ => (0, 1),
            Direction::PrevI => (-1, 0),
            Direction::NextI => (1, 0),
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

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Square {}: ni = {:?}, nj = {:?}, pi = {:?}, pj = {:?}",
            self.id,
            self.next_i.map(|n| n.id),
            self.next_j.map(|n| n.id),
            self.prev_i.map(|n| n.id),
            self.prev_j.map(|n| n.id)
        )
    }
}
