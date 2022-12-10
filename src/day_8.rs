use crate::data::Data;
use crate::DayOutput;
use itertools::Itertools;

#[derive(Debug)]
struct Tree {
    height: usize,
    visible: bool,
    scenic_score: usize,
}

pub fn solve(data: &Data) -> DayOutput {
    let mut trees = data
        .lines()
        .map(|line| {
            line.iter()
                .map(|b| Tree {
                    height: (b - b'0') as usize,
                    visible: false,
                    scenic_score: 1,
                })
                .collect_vec()
        })
        .collect_vec();

    let cols = trees[0].len();

    for row in &mut trees {
        handle_line(row.iter_mut());
        handle_line(row.iter_mut().rev());
    }
    for j in 0..cols {
        handle_line(trees.iter_mut().map(|row| &mut row[j]));
        handle_line(trees.iter_mut().rev().map(|row| &mut row[j]));
    }

    let num_visible = trees.iter().flatten().filter(|tree| tree.visible).count();
    let best_scenic_score = trees
        .iter()
        .flatten()
        .map(|tree| tree.scenic_score)
        .max()
        .unwrap();

    (num_visible as i64, best_scenic_score as i64).into()
}

fn handle_line<'a>(trees: impl Iterator<Item = &'a mut Tree>) {
    let mut max_height = 0;
    let mut last_index_by_height = [0; 10];

    for (i, tree) in trees.enumerate() {
        let line_of_sight;
        if i == 0 || tree.height > max_height {
            tree.visible = true;
            max_height = tree.height;
            line_of_sight = i;
        } else {
            let closest_non_smaller_tree =
                *last_index_by_height[tree.height..].iter().max().unwrap();
            line_of_sight = i - closest_non_smaller_tree;
        }

        tree.scenic_score *= line_of_sight;
        last_index_by_height[tree.height] = i;
    }
}
