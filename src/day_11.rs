use crate::data::{Data, ParseBytes};
use crate::iter_utils::IterUtils;
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Monkey<I> {
    items: Vec<I>,
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

#[derive(Debug, Clone)]
struct RemainderItem {
    remainder_values: Vec<RemainderValue>,
}

#[derive(Debug, Clone, Copy)]
struct RemainderValue {
    module: i64,
    remainder: i64,
}

pub fn solve(data: &Data) -> DayOutput {
    let monkeys_part_1 = data
        .paragraphs()
        .enumerate()
        .map(|(i, lines)| {
            let lines: [_; 6] = lines.lines().collect_vec().try_into().unwrap();
            let [header, mut starting_items, mut operation, mut test, mut if_true, mut if_false] =
                lines;

            assert_eq!(header, format!("Monkey {}:", i).as_bytes());

            starting_items.consume_prefix(b"  Starting items: ");
            let items = starting_items
                .split_bytes(b", ", false)
                .parsed::<i64>()
                .collect();

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

    let modules = monkeys_part_1
        .iter()
        .map(|monkey| monkey.borrow().test_divisible)
        .collect_vec();
    let monkeys_part_2 = monkeys_part_1
        .iter()
        .map(|monkey| {
            let monkey = monkey.borrow();
            RefCell::new(Monkey {
                items: monkey
                    .items
                    .iter()
                    .map(|&item| RemainderItem {
                        remainder_values: modules
                            .iter()
                            .map(|&module| RemainderValue {
                                module,
                                remainder: item % module,
                            })
                            .collect(),
                    })
                    .collect(),
                inspection: monkey.inspection,
                test_divisible: monkey.test_divisible,
                if_true: monkey.if_true,
                if_false: monkey.if_false,
                business: monkey.business,
            })
        })
        .collect_vec();

    let part_1 = do_rounds_part_1(monkeys_part_1, 20, 3);
    let part_2 = do_rounds_part_2(monkeys_part_2, 10_000);

    (part_1, part_2).into()
}

fn do_rounds_part_1(monkeys: Vec<RefCell<Monkey<i64>>>, num_rounds: i32, dumping: i64) -> i64 {
    for _ in 0..num_rounds {
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

    (business[0] * business[1]) as i64
}

fn do_rounds_part_2(monkeys: Vec<RefCell<Monkey<RemainderItem>>>, num_rounds: i32) -> i64 {
    for _ in 0..num_rounds {
        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            let monkey = &mut *monkey;
            monkey.business += monkey.items.len();
            for mut item in monkey.items.drain(..) {
                let mut test_remainder = None;
                for remainder in &mut item.remainder_values {
                    let new_remainder = match monkey.inspection {
                        Inspection::Add(n) => remainder.remainder + n,
                        Inspection::Multiply(n) => remainder.remainder * n,
                        Inspection::Square => remainder.remainder * remainder.remainder,
                    };
                    remainder.remainder = new_remainder % remainder.module;

                    if remainder.module == monkey.test_divisible {
                        test_remainder = Some(remainder.remainder);
                    }
                }

                let target = if test_remainder.unwrap() == 0 {
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

    (business[0] * business[1]) as i64
}
