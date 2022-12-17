use crate::data::{Data, ParseBytes, TryFromBytes};
use crate::iter_utils::IterUtils;
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

pub fn solve(data: &Data) -> DayOutput {
    const MAX_X: usize = 700;
    const MAX_Y: usize = 200;

    let mut tiles = (0..MAX_X).map(|_| vec![Tile::Air; MAX_Y]).collect_vec();
    let mut max_y_rock = 0;

    for path in data.lines() {
        let points = path
            .split_bytes(b" -> ", false)
            .parsed::<Point>()
            .inspect(|point| {
                max_y_rock = max_y_rock.max(point.y);
            });

        for (a, b) in points.tuple_windows::<(_, _)>() {
            if a.x == b.x {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    tiles[a.x][y] = Tile::Rock;
                }
            } else if a.y == b.y {
                for column in &mut tiles[a.x.min(b.x)..=a.x.max(b.x)] {
                    column[a.y] = Tile::Rock;
                }
            } else {
                unreachable!()
            }
        }
    }

    let mut added_sand = 0;
    loop {
        if !add_sand(&mut tiles) {
            break;
        }
        added_sand += 1;
    }
    let part_1 = added_sand;

    // Add "infinite" rock layer
    for column in &mut tiles {
        column[max_y_rock + 2] = Tile::Rock;
    }
    loop {
        add_sand(&mut tiles);
        added_sand += 1;
        if tiles[500][0] == Tile::Sand {
            break;
        }
    }

    (part_1, added_sand).into()
}

fn add_sand(tiles: &mut [Vec<Tile>]) -> bool {
    let max_y = tiles[0].len() - 1;
    let mut sand_x = 500;
    let mut sand_y = 0;

    while sand_y < max_y {
        if tiles[sand_x][sand_y + 1] == Tile::Air {
            sand_y += 1;
        } else if tiles[sand_x - 1][sand_y + 1] == Tile::Air {
            sand_x -= 1;
            sand_y += 1;
        } else if tiles[sand_x + 1][sand_y + 1] == Tile::Air {
            sand_x += 1;
            sand_y += 1;
        } else {
            tiles[sand_x][sand_y] = Tile::Sand;
            return true;
        }
    }

    false
}

impl TryFromBytes for Point {
    fn try_from_bytes(mut bytes: &[u8]) -> Option<Self> {
        let x = bytes.consume_until(b',').parse_bytes();
        let y = bytes.parse_bytes();
        Some(Point { x, y })
    }
}
