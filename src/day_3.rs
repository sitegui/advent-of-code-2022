use crate::data::Data;
use itertools::Itertools;
use std::mem;

pub fn solve() -> (i64, i64) {
    Data::read(3)
        .lines()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            let same = find_same_item_2(first_half, second_half);

            (line, priority(same))
        })
        .tuples::<(_, _, _)>()
        .into_iter()
        .map(|(elf_1, elf_2, elf_3)| {
            let badge = find_same_item_3(elf_1.0, elf_2.0, elf_3.0);

            (elf_1.1 + elf_2.1 + elf_3.1, priority(badge))
        })
        .fold((0, 0), |acc, round| (acc.0 + round.0, acc.1 + round.1))
}

fn find_same_item_2(items_a: &[u8], items_b: &[u8]) -> u8 {
    for &item in items_a {
        if items_b.contains(&item) {
            return item;
        }
    }

    unreachable!()
}

fn find_same_item_3(items_a: &[u8], items_b: &[u8], items_c: &[u8]) -> u8 {
    for &item in items_a {
        if items_b.contains(&item) && items_c.contains(&item) {
            return item;
        }
    }

    unreachable!()
}

fn priority(item: u8) -> i64 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as i64,
        b'A'..=b'Z' => (item - b'A' + 27) as i64,
        _ => unreachable!(),
    }
}
