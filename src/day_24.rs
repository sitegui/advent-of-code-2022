use crate::data::Data;
use crate::xy::{Xy, Xyz};
use crate::DayOutput;
use pathfinding::prelude::astar;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    pos: Xy,
    direction: Xy,
}

/// Stores the blizzards' position in the last frame and a set for each past frame
#[derive(Debug)]
struct BlizzardsCache {
    blizzards: Vec<Blizzard>,
    top_left: Xy,
    bottom_right: Xy,
    /// `hashes[time_step]` contains the (unique) positions of the blizzards at a given frame
    hashes: Vec<HashSet<Xy>>,
}

pub fn solve(data: &Data) -> DayOutput {
    let blizzards = Rc::new(RefCell::new(BlizzardsCache::parse(data)));
    let goal = blizzards.borrow().bottom_right + Xy::y_axis();

    let first_crossing = search(&blizzards, Xyz::new(1, 0, 0), goal);
    let second_crossing = search(&blizzards, first_crossing, Xy::new(1, 0));
    let third_crossing = search(&blizzards, second_crossing, goal);

    DayOutput::Int(first_crossing.z as i64, third_crossing.z as i64)
}

fn search(blizzards: &Rc<RefCell<BlizzardsCache>>, start: Xyz, goal: Xy) -> Xyz {
    let (path, _) = astar(
        &start,
        |&pos| NeighborsIter::new(pos, blizzards.clone()),
        |&pos| pos.xy().manhattan_distance(goal),
        |&pos| pos.xy() == goal,
    )
    .unwrap();

    path[path.len() - 1]
}

impl BlizzardsCache {
    fn parse(data: &Data) -> Self {
        let mut blizzards = Vec::new();
        for (y, line) in data.lines().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let direction = match c {
                    b'>' => Xy::new(1, 0),
                    b'<' => Xy::new(-1, 0),
                    b'^' => Xy::new(0, -1),
                    b'v' => Xy::new(0, 1),
                    _ => continue,
                };

                blizzards.push(Blizzard {
                    pos: Xy::new(x as i32, y as i32),
                    direction,
                });
            }
        }

        let top_left = Xy::new(1, 1);
        let bottom_right = Xy::new(
            data.lines().next().unwrap().len() as i32 - 2,
            data.bytes().iter().filter(|&&c| c == b'\n').count() as i32 - 2,
        );

        BlizzardsCache {
            top_left,
            bottom_right,
            hashes: vec![blizzards.iter().map(|blizzard| blizzard.pos).collect()],
            blizzards,
        }
    }

    fn advance(&mut self) {
        for blizzard in &mut self.blizzards {
            blizzard.pos += blizzard.direction;

            if blizzard.pos.x < self.top_left.x {
                blizzard.pos.x = self.bottom_right.x;
            } else if blizzard.pos.x > self.bottom_right.x {
                blizzard.pos.x = self.top_left.x;
            } else if blizzard.pos.y < self.top_left.y {
                blizzard.pos.y = self.bottom_right.y;
            } else if blizzard.pos.y > self.bottom_right.y {
                blizzard.pos.y = self.top_left.y;
            }
        }

        self.hashes
            .push(self.blizzards.iter().map(|blizzard| blizzard.pos).collect());
    }

    fn contains(&mut self, pos: Xyz) -> bool {
        let time = pos.z as usize;
        while self.hashes.len() <= time {
            self.advance();
        }

        self.hashes[time].contains(&pos.xy())
    }

    /// Check that the position is inbound and that there is no blizzard on it
    fn is_valid_pos(&mut self, pos: Xyz) -> bool {
        let start = self.top_left - Xy::y_axis();
        let goal = self.bottom_right + Xy::y_axis();

        if pos.xy() == start || pos.xy() == goal {
            return true;
        }

        pos.x >= self.top_left.x
            && pos.x <= self.bottom_right.x
            && pos.y >= self.top_left.y
            && pos.y <= self.bottom_right.y
            && !self.contains(pos)
    }
}

#[derive(Debug)]
struct NeighborsIter {
    index: usize,
    pos: Xyz,
    blizzards: Rc<RefCell<BlizzardsCache>>,
}

impl NeighborsIter {
    const DELTAS: [Xyz; 5] = [
        Xyz::new(1, 0, 1),
        Xyz::new(-1, 0, 1),
        Xyz::new(0, 1, 1),
        Xyz::new(0, -1, 1),
        Xyz::new(0, 0, 1),
    ];

    fn new(pos: Xyz, blizzards: Rc<RefCell<BlizzardsCache>>) -> Self {
        NeighborsIter {
            index: 0,
            pos,
            blizzards,
        }
    }
}

impl Iterator for NeighborsIter {
    type Item = (Xyz, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut blizzards = self.blizzards.borrow_mut();
        while let Some(&delta) = Self::DELTAS.get(self.index) {
            self.index += 1;
            let neighbor = delta + self.pos;
            if blizzards.is_valid_pos(neighbor) {
                return Some((neighbor, 1));
            }
        }

        None
    }
}
