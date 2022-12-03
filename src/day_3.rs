use crate::data::Data;
use itertools::Itertools;

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
    let mut in_a = [false; 255];
    let in_b = hash_items(items_b);

    for &item in items_a {
        if !in_a[item as usize] {
            in_a[item as usize] = true;

            if in_b[item as usize] {
                return item;
            }
        }
    }

    unreachable!()
}

fn find_same_item_3(items_a: &[u8], items_b: &[u8], items_c: &[u8]) -> u8 {
    let mut in_a = [false; 255];
    let in_b = hash_items(items_b);
    let in_c = hash_items(items_c);

    for &item in items_a {
        if !in_a[item as usize] {
            in_a[item as usize] = true;

            if in_b[item as usize] && in_c[item as usize] {
                return item;
            }
        }
    }

    unreachable!()
}

fn hash_items(items: &[u8]) -> [bool; 255] {
    let mut hash = [false; 255];
    for &item in items {
        hash[item as usize] = true;
    }
    hash
}

fn priority(item: u8) -> i64 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as i64,
        b'A'..=b'Z' => (item - b'A' + 27) as i64,
        _ => unreachable!(),
    }
}
