use crate::data::Data;
use crate::iter_utils::IterUtils;
use crate::parser::Parser;
use itertools::Itertools;
use std::ops::RangeInclusive;

pub fn solve(data: &Data) -> (i64, i64) {
    data.lines()
        .map(|line| {
            let (elf_1, elf_2) = line
                .split_byte(b',', true)
                .map(parse_range)
                .collect_tuple::<(_, _)>()
                .unwrap();

            let part_1 = is_fully_contained(&elf_1, &elf_2) || is_fully_contained(&elf_2, &elf_1);
            let part_2 = !doesnt_overlap(&elf_1, &elf_2);

            (part_1 as i64, part_2 as i64)
        })
        .fold((0, 0), |acc, round| (acc.0 + round.0, acc.1 + round.1))
}

fn parse_range(bytes: &[u8]) -> RangeInclusive<i32> {
    let (a, b) = bytes
        .split_byte(b'-', true)
        .parsed::<i32>()
        .collect_tuple()
        .unwrap();

    a..=b
}

fn is_fully_contained(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn doesnt_overlap(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() > b.end() || b.start() > a.end()
}
