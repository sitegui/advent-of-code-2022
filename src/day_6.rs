use crate::data::Data;
use crate::DayOutput;

pub fn solve(data: &Data) -> DayOutput {
    let p1 = find_start(data.bytes(), 4) as i64 + 4;
    let p2 = find_start(data.bytes(), 14) as i64 + 14;

    (p1, p2).into()
}

fn find_start(bytes: &[u8], n: usize) -> usize {
    'search: for (offset, window) in bytes.windows(n).enumerate() {
        for i in 0..n - 1 {
            for j in i + 1..n {
                if window[i] == window[j] {
                    continue 'search;
                }
            }
        }

        return offset;
    }

    unreachable!()
}
