use crate::data::{Data, ParseBytes, TryFromBytes};
use crate::iter_utils::IterUtils;
use crate::parser::Parser;
use crate::DayOutput;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{Finish, IResult};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Values {
    values: Vec<Value>,
}

#[derive(Debug, Eq, PartialEq)]
enum Value {
    Int(i32),
    List(Values),
}

pub fn solve(data: &Data) -> DayOutput {
    let divider_1 = Values {
        values: vec![Value::List(Values {
            values: vec![Value::Int(2)],
        })],
    };
    let divider_2 = Values {
        values: vec![Value::List(Values {
            values: vec![Value::Int(6)],
        })],
    };

    let mut part_1 = 0;
    let mut less_than_divider_1 = 0;
    let mut less_than_divider_2 = 0;
    for (i, paragraph) in data.paragraphs().enumerate() {
        let (a, b) = paragraph
            .lines()
            .parsed()
            .collect_tuple::<(Values, Values)>()
            .unwrap();

        if a <= b {
            part_1 += i as i64 + 1;
        }

        if a < divider_2 {
            less_than_divider_2 += 1;
            if a < divider_1 {
                less_than_divider_1 += 1;
            }
        }
        if b < divider_2 {
            less_than_divider_2 += 1;
            if b < divider_1 {
                less_than_divider_1 += 1;
            }
        }
    }

    let part_2 = (less_than_divider_1 + 1) * (less_than_divider_2 + 2);

    (part_1, part_2).into()
}

impl TryFromBytes for Values {
    fn try_from_bytes(bytes: &[u8]) -> Option<Self> {
        fn parse_values(input: &[u8]) -> IResult<&[u8], Values> {
            delimited(
                tag(b"["),
                map(separated_list0(tag(b","), parse_value), |values| Values {
                    values,
                }),
                tag(b"]"),
            )(input)
        }

        fn parse_value(input: &[u8]) -> IResult<&[u8], Value> {
            alt((
                map(digit1, |bytes: &[u8]| Value::Int(bytes.parse_bytes())),
                map(parse_values, Value::List),
            ))(input)
        }

        all_consuming(parse_values)(bytes)
            .finish()
            .ok()
            .map(|(_, values)| values)
    }
}

impl Ord for Values {
    fn cmp(&self, other: &Self) -> Ordering {
        self.values.cmp(&other.values)
    }
}

impl PartialOrd for Values {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::List(a), Value::Int(b)) => {
                let b_values = Values {
                    values: vec![Value::Int(*b)],
                };
                a.cmp(&b_values)
            }
            (Value::Int(a), Value::List(b)) => {
                let a_values = Values {
                    values: vec![Value::Int(*a)],
                };
                a_values.cmp(b)
            }
            (Value::List(a), Value::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
