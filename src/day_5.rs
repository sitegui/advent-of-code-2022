use crate::data::{Data, ParseBytes};
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;
use std::cell::RefCell;

pub fn solve(data: &Data) -> DayOutput {
    let (stacks_paragraph, movements) = data.paragraphs().collect_tuple::<(_, _)>().unwrap();
    let mut stack_paragraph = stacks_paragraph.lines().collect_vec();

    let labels = stack_paragraph.pop().unwrap();
    let num_stacks = (labels.len() + 1) / 4;
    let mut stacks_p1 = Vec::with_capacity(num_stacks);
    for i in 0..num_stacks {
        let mut stack = Vec::with_capacity(stack_paragraph.len() * num_stacks);

        // Read from bottom to top
        for &row in stack_paragraph.iter().rev() {
            let crate_id = row[4 * i + 1];
            if crate_id != b' ' {
                stack.push(crate_id);
            }
        }
        stacks_p1.push(RefCell::new(stack));
    }
    let stacks_p2 = stacks_p1.clone();

    for mut movement in movements.lines() {
        movement.consume_prefix(b"move ");
        let amount: usize = movement.consume_until(b' ').parse_bytes();
        movement.consume_prefix(b"from ");
        let origin: usize = movement.consume_until(b' ').parse_bytes();
        movement.consume_prefix(b"to ");
        let destination: usize = movement.parse_bytes();

        {
            let mut origin = stacks_p1[origin - 1].borrow_mut();
            let mut destination = stacks_p1[destination - 1].borrow_mut();
            let range = origin.len() - amount..;
            destination.extend(origin.drain(range).rev());
        }

        {
            let mut origin = stacks_p2[origin - 1].borrow_mut();
            let mut destination = stacks_p2[destination - 1].borrow_mut();
            let range = origin.len() - amount..;
            destination.extend(origin.drain(range));
        }
    }

    let part_1: String = stacks_p1
        .iter()
        .filter_map(|stack| stack.borrow().last().copied())
        .map(|c| c as char)
        .collect();

    let part_2: String = stacks_p2
        .iter()
        .filter_map(|stack| stack.borrow().last().copied())
        .map(|c| c as char)
        .collect();

    (part_1, part_2).into()
}
