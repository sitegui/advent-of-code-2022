use crate::data::Data;
use crate::nom_parser::*;
use crate::DayOutput;
use nom::AsBytes;
use std::collections::{HashMap, HashSet};

type Name = [u8; 4];
type Monkeys = HashMap<Name, Monkey>;

const HUMAN: Name = *b"humn";
const ROOT: Name = *b"root";

#[derive(Debug, Copy, Clone)]
struct Monkey {
    name: Name,
    operation: Operation,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Value(i64),
    Add(Name, Name),
    Sub(Name, Name),
    Mul(Name, Name),
    Div(Name, Name),
}

pub fn solve(data: &Data) -> DayOutput {
    let monkeys = lines(parse_monkey).consume_all(data.bytes());
    let monkeys: HashMap<_, _> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name, monkey))
        .collect();

    let part_1 = calculate(&monkeys, ROOT);

    let human_dependent = find_human_dependent(&monkeys, ROOT);
    let (operator_1, operator_2) = monkeys[&ROOT].operation.operators().unwrap();
    let (root_branch, target) = if human_dependent.contains(&operator_1) {
        (operator_1, calculate(&monkeys, operator_2))
    } else {
        (operator_2, calculate(&monkeys, operator_1))
    };
    let part_2 = solve_for_human(&monkeys, &human_dependent, root_branch, target);

    (part_1, part_2).into()
}

fn calculate(monkeys: &Monkeys, target: Name) -> i64 {
    let monkey = &monkeys[&target];

    match monkey.operation {
        Operation::Value(value) => value,
        Operation::Add(operator_1, operator_2) => {
            calculate(monkeys, operator_1) + calculate(monkeys, operator_2)
        }
        Operation::Sub(operator_1, operator_2) => {
            calculate(monkeys, operator_1) - calculate(monkeys, operator_2)
        }
        Operation::Mul(operator_1, operator_2) => {
            calculate(monkeys, operator_1) * calculate(monkeys, operator_2)
        }
        Operation::Div(operator_1, operator_2) => {
            calculate(monkeys, operator_1) / calculate(monkeys, operator_2)
        }
    }
}

/// Return the names of all monkeys that depend on [`HUMAN`].
fn find_human_dependent(monkeys: &Monkeys, root: Name) -> HashSet<Name> {
    let mut result = HashSet::new();

    fn recurse(monkeys: &Monkeys, result: &mut HashSet<Name>, current: Name) -> bool {
        let depends_on_human = match (current, monkeys[&current].operation) {
            (HUMAN, _) => true,
            (_, Operation::Value(_)) => false,
            (
                _,
                Operation::Add(operator_1, operator_2)
                | Operation::Sub(operator_1, operator_2)
                | Operation::Mul(operator_1, operator_2)
                | Operation::Div(operator_1, operator_2),
            ) => recurse(monkeys, result, operator_1) || recurse(monkeys, result, operator_2),
        };

        if depends_on_human {
            result.insert(current);
        }

        depends_on_human
    }

    recurse(monkeys, &mut result, root);

    result
}

fn solve_for_human(
    monkeys: &Monkeys,
    human_dependent: &HashSet<Name>,
    current: Name,
    target_value: i64,
) -> i64 {
    if current == HUMAN {
        return target_value;
    }

    let operation = monkeys[&current].operation;
    let (operator_1, operator_2) = operation.operators().unwrap();

    if human_dependent.contains(&operator_1) {
        let value_2 = calculate(monkeys, operator_2);
        let sub_target = operation.isolate_left(target_value, value_2);
        solve_for_human(monkeys, human_dependent, operator_1, sub_target)
    } else {
        let value_1 = calculate(monkeys, operator_1);
        let sub_target = operation.isolate_right(target_value, value_1);
        solve_for_human(monkeys, human_dependent, operator_2, sub_target)
    }
}

fn parse_monkey(input: &[u8]) -> PResult<Monkey> {
    map(
        tuple((parse_name, tag(b": "), parse_operation)),
        |(name, _, operation)| Monkey { name, operation },
    )(input)
}

fn parse_name(input: &[u8]) -> PResult<Name> {
    map(take(4usize), |name: &[u8]| name.try_into().unwrap())(input)
}

fn parse_operation(input: &[u8]) -> PResult<Operation> {
    alt((
        map(nom_i64, Operation::Value),
        map(
            tuple((
                parse_name,
                tag(b" "),
                one_of(b"+-*/".as_bytes()),
                tag(b" "),
                parse_name,
            )),
            |(a, _, op, _, b)| match op {
                '+' => Operation::Add(a, b),
                '-' => Operation::Sub(a, b),
                '*' => Operation::Mul(a, b),
                '/' => Operation::Div(a, b),
                _ => unreachable!(),
            },
        ),
    ))(input)
}

impl Operation {
    fn operators(self) -> Option<(Name, Name)> {
        match self {
            Operation::Value(_) => None,
            Operation::Add(operator_1, operator_2)
            | Operation::Sub(operator_1, operator_2)
            | Operation::Mul(operator_1, operator_2)
            | Operation::Div(operator_1, operator_2) => Some((operator_1, operator_2)),
        }
    }

    fn isolate_left(self, target: i64, branch: i64) -> i64 {
        match self {
            Operation::Add(_, _) => target - branch,
            Operation::Sub(_, _) => target + branch,
            Operation::Mul(_, _) => target / branch,
            Operation::Div(_, _) => target * branch,
            _ => unreachable!(),
        }
    }

    fn isolate_right(self, target: i64, branch: i64) -> i64 {
        match self {
            Operation::Add(_, _) => target - branch,
            Operation::Sub(_, _) => branch - target,
            Operation::Mul(_, _) => target / branch,
            Operation::Div(_, _) => branch / target,
            _ => unreachable!(),
        }
    }
}
