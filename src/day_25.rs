use crate::data::Data;
use crate::DayOutput;

pub fn solve(data: &Data) -> DayOutput {
    let mut total = 0;
    for line in data.lines() {
        total += snafu_to_dec(line);
    }
    let part_1 = dec_to_snafu(total);

    DayOutput::Str(part_1, "".to_string())
}

fn snafu_to_dec(code: &[u8]) -> i64 {
    let mut value = 0;
    for (exp, c) in code.iter().rev().enumerate() {
        let factor = 5i64.pow(exp as u32);
        let digit = match c {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!(),
        };
        value += digit * factor;
    }
    value
}

fn dec_to_snafu(mut value: i64) -> String {
    let mut code = String::new();
    while value != 0 {
        let (c, digit) = match value % 5 {
            0 => ('0', 0),
            1 => ('1', 1),
            2 => ('2', 2),
            3 => ('=', -2),
            4 => ('-', -1),
            _ => unreachable!(),
        };

        code.push(c);

        value -= digit;
        value /= 5;
    }
    code.chars().rev().collect()
}
