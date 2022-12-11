use crate::data::{Data, ParseBytes};
use crate::parser::Parser;
use crate::DayOutput;

pub fn solve(data: &Data) -> DayOutput {
    let mut cycle = 1;
    let mut register = 1;
    let mut total_strength = 0;
    let mut screen = Vec::with_capacity(40 * 6);

    let mut do_cycle = |register_delta: i64| {
        if (cycle - 20) % 40 == 0 {
            total_strength += cycle * register;
        }

        let crt_x = (cycle - 1) % 40;
        let pixel = if crt_x >= register - 1 && crt_x <= register + 1 {
            '#'
        } else {
            '.'
        };
        screen.push(pixel);

        cycle += 1;
        register += register_delta;
    };

    for mut line in data.lines() {
        if line.try_consume_prefix(b"addx ").is_some() {
            let value = line.parse_bytes::<i64>();
            do_cycle(0);
            do_cycle(value);
        } else if line == b"noop" {
            do_cycle(0);
        } else {
            unreachable!()
        }
    }

    // Manual inspection
    // println!(
    //     "{}",
    //     screen
    //         .chunks(40)
    //         .format_with("\n", |row, f| { f(&row.iter().format("")) })
    // );

    (total_strength, "ZFBFHGUP").into()
}
