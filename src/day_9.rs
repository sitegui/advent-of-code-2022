use crate::data::{Data, ParseBytes};
use crate::parser::Parser;
use crate::DayOutput;
use std::collections::HashSet;

pub fn solve(data: &Data) -> DayOutput {
    const NUM_KNOTS: usize = 10;

    // knots[0] is the head
    let mut knots = [(0, 0); NUM_KNOTS];
    let mut visited_by_1 = HashSet::new();
    let mut visited_by_9 = HashSet::new();
    visited_by_1.insert(knots[1]);
    visited_by_9.insert(knots[9]);

    for mut line in data.lines() {
        let direction = match line.consume_word() {
            b"R" => (1, 0),
            b"L" => (-1, 0),
            b"D" => (0, 1),
            b"U" => (0, -1),
            _ => unreachable!(),
        };

        let amount = line.parse_bytes::<i32>();
        for _ in 0..amount {
            knots[0].0 += direction.0;
            knots[0].1 += direction.1;

            for i in 1..NUM_KNOTS {
                let base = knots[i - 1];
                let moving = &mut knots[i];
                let movement = match (moving.0 - base.0, moving.1 - base.1) {
                    (2, 0) => (-1, 0),
                    (-2, 0) => (1, 0),
                    (0, 2) => (0, -1),
                    (0, -2) => (0, 1),
                    (2, 1) | (2, 2) | (1, 2) => (-1, -1),
                    (-1, 2) | (-2, 2) | (-2, 1) => (1, -1),
                    (-2, -1) | (-2, -2) | (-1, -2) => (1, 1),
                    (1, -2) | (2, -2) | (2, -1) => (-1, 1),
                    _ => continue,
                };

                moving.0 += movement.0;
                moving.1 += movement.1;

                if i == 1 {
                    visited_by_1.insert(knots[1]);
                } else if i == 9 {
                    visited_by_9.insert(knots[9]);
                }
            }
        }
    }

    (visited_by_1.len() as i64, visited_by_9.len() as i64).into()
}
