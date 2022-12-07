use crate::data::{Data, ParseBytes};
use crate::parser::Parser;
use crate::DayOutput;
use std::collections::HashMap;

pub fn solve(data: &Data) -> DayOutput {
    let mut cwd = vec![];
    let mut size_by_dir: HashMap<_, i64> = HashMap::new();

    for mut line in data.lines() {
        if line.try_consume_prefix(b"$ cd ").is_some() {
            if line == b"/" {
                cwd = vec![];
            } else if line == b".." {
                cwd.pop().unwrap();
            } else {
                cwd.push(line);
            }
        } else if line != b"$ ls" && !line.starts_with(b"dir ") {
            let size: i64 = line.consume_words(1).parse_bytes();

            for i in 0..=cwd.len() {
                *size_by_dir.entry(cwd[0..i].to_vec()).or_default() += size;
            }
        }
    }

    let part_1 = size_by_dir.values().filter(|&&size| size <= 100_000).sum();

    let min_delete_size = *size_by_dir.get([].as_slice()).unwrap() - 40_000_000;
    let part_2 = *size_by_dir
        .values()
        .filter(|&&size| size >= min_delete_size)
        .min()
        .unwrap();

    (part_1, part_2).into()
}
