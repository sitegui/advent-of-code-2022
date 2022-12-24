use crate::data::Data;
use crate::nom_parser::*;
use crate::DayOutput;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Item {
    initial_position: i32,
    value: i64,
}

pub fn solve(data: &Data) -> DayOutput {
    let mut part_1_items: Vec<_> = lines(nom_i64)
        .consume_all(data.bytes())
        .into_iter()
        .enumerate()
        .map(|(i, value)| Item {
            initial_position: i as i32,
            value,
        })
        .collect();

    let mut part_2_items = part_1_items.clone();

    mix_items(&mut part_1_items);
    let part_1 = groove_coordinates(part_1_items);

    for item in &mut part_2_items {
        item.value *= 811_589_153;
    }
    for _ in 0..10 {
        mix_items(&mut part_2_items);
    }
    let part_2 = groove_coordinates(part_2_items);

    (part_1, part_2).into()
}

fn groove_coordinates(items: Vec<Item>) -> i64 {
    let zero_position = items.iter().position(|item| item.value == 0).unwrap();
    let a = items[(zero_position + 1_000).rem_euclid(items.len())].value;
    let b = items[(zero_position + 2_000).rem_euclid(items.len())].value;
    let c = items[(zero_position + 3_000).rem_euclid(items.len())].value;
    a + b + c
}

fn mix_items(items: &mut [Item]) {
    let num_items = items.len() as i64;

    for initial_position in 0..num_items {
        let (current_index, &item) = items
            .iter()
            .find_position(|item| item.initial_position == initial_position as i32)
            .unwrap();
        let new_index = (current_index as i64 + item.value).rem_euclid(num_items - 1) as usize;

        #[allow(clippy::comparison_chain)]
        if current_index < new_index {
            items[current_index..new_index + 1].rotate_left(1);
        } else if current_index > new_index {
            items[new_index..current_index + 1].rotate_right(1);
        }
    }
}
