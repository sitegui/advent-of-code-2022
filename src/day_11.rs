use crate::data::{Data, ParseBytes};
use crate::iter_utils::IterUtils;
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    inspection: Inspection,
    test_divisible: i64,
    if_true: usize,
    if_false: usize,
    business: usize,
}

#[derive(Debug, Copy, Clone)]
enum Inspection {
    Add(i64),
    Multiply(i64),
    Square,
}

pub fn solve(data: &Data) -> DayOutput {
    let monkeys = data
        .paragraphs()
        .enumerate()
        .map(|(i, lines)| {
            let lines: [_; 6] = lines.lines().collect_vec().try_into().unwrap();
            let [header, mut starting_items, mut operation, mut test, mut if_true, mut if_false] =
                lines;

            assert_eq!(header, format!("Monkey {}:", i).as_bytes());

            starting_items.consume_prefix(b"  Starting items: ");
            let items = starting_items.split_bytes(b", ", false).parsed().collect();

            operation.consume_prefix(b"  Operation: new = old ");
            let inspection = match operation.consume_bytes(2) {
                b"+ " => Inspection::Add(operation.parse_bytes()),
                b"* " => {
                    if operation == b"old" {
                        Inspection::Square
                    } else {
                        Inspection::Multiply(operation.parse_bytes())
                    }
                }
                _ => unreachable!(),
            };

            test.consume_prefix(b"  Test: divisible by ");
            let test_divisible = test.parse_bytes();

            if_true.consume_prefix(b"    If true: throw to monkey ");
            if_false.consume_prefix(b"    If false: throw to monkey ");

            RefCell::new(Monkey {
                items,
                inspection,
                test_divisible,
                if_true: if_true.parse_bytes(),
                if_false: if_false.parse_bytes(),
                business: 0,
            })
        })
        .collect_vec();

    let part_1 = do_rounds(monkeys.clone(), 20, 3);
    let part_2 = do_rounds(monkeys, 10_000, 1);

    (part_1, part_2).into()
}

fn do_rounds(monkeys: Vec<RefCell<Monkey>>, num_rounds: i32, dumping: i64) -> i64 {
    for i in 0..num_rounds {
        if i % 1000 == 1 {
            eprintln!(
                "business = {}",
                monkeys
                    .iter()
                    .map(|monkey| monkey.borrow().business)
                    .format(", ")
            );
        }

        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            let monkey = &mut *monkey;
            monkey.business += monkey.items.len();
            for mut item in monkey.items.drain(..) {
                item = match monkey.inspection {
                    Inspection::Add(n) => item + n,
                    Inspection::Multiply(n) => item * n,
                    Inspection::Square => item * item,
                };
                item /= dumping;
                let target = if item % monkey.test_divisible == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                monkeys[target].borrow_mut().items.push(item);
            }
        }
    }

    let business = monkeys
        .iter()
        .map(|monkey| monkey.borrow().business)
        .sorted()
        .rev()
        .collect_vec();

    eprintln!("business = {:?}", business);
    (business[0] * business[1]) as i64
}
