use crate::data::Data;
use crate::iter_utils::IterUtils;
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;
use std::cmp::Reverse;

pub fn solve(data: &Data) -> DayOutput {
    let elves = data
        .paragraphs()
        .map(|paragraph| paragraph.lines().parsed::<i64>().sum::<i64>())
        .sorted_by_key(|&i| Reverse(i))
        .collect_vec();

    let max = elves[0];
    let top_3 = elves[0..3].iter().sum::<i64>();

    (max, top_3).into()
}
