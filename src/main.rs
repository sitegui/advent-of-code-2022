mod data;
mod iter_utils;
mod parser;

use std::env::args;
use std::fmt;
use std::time::{Duration, Instant};

const NUM_WARMING: usize = 2;
const NUM_SAMPLES: usize = 5;

struct Day {
    label: &'static str,
    solve_fn: fn() -> (i64, i64),
    expected: (i64, i64),
}

#[derive(Clone)]
struct BenchResult {
    label: String,
    samples: Vec<Duration>,
    mean: Duration,
    std: Duration,
}

impl Day {
    fn solve(&self) -> (i64, i64) {
        (self.solve_fn)()
    }

    fn bench_solve(&self) -> BenchResult {
        for _ in 0..NUM_WARMING {
            self.assert_solve();
        }

        let mut samples = Vec::with_capacity(10);
        for _ in 0..NUM_SAMPLES {
            let start = Instant::now();
            self.assert_solve();
            samples.push(start.elapsed());
        }

        BenchResult::from_samples(self.label, samples)
    }

    fn assert_solve(&self) {
        let answer = self.solve();
        assert_eq!(answer, self.expected, "when checking {}", self.label);
    }
}

impl BenchResult {
    fn from_samples(label: impl Into<String>, samples: Vec<Duration>) -> Self {
        let mean = samples.iter().sum::<Duration>() / samples.len() as u32;
        let std = (samples
            .iter()
            .map(|&s| (s.as_secs_f64() - mean.as_secs_f64()).powi(2))
            .sum::<f64>()
            / (samples.len() as f64 - 1.))
            .sqrt();

        BenchResult {
            label: label.into(),
            samples,
            mean,
            std: Duration::from_secs_f64(std),
        }
    }
}

macro_rules! days {
    ($($day:ident = ($part_1:expr, $part_2:expr)),* $(,)?) => {
        $(mod $day;)*

        const DAYS: &[Day] = &[
            $(Day {
                label: stringify!($day),
                solve_fn: $day::solve,
                expected: ($part_1, $part_2)
            }),*
        ];
    };
}

days! {
    day_1 = (68467, 203420),
    day_2 = (15422, 15442),
}

fn main() {
    match args().nth(1) {
        None => {
            println!("Will execute all days to time their individual and total execution times");

            let mut results = Vec::with_capacity(DAYS.len());
            for day in DAYS {
                let result = day.bench_solve();
                println!("{}", result);
                results.push(result);
            }

            let combined: Vec<Duration> = (0..NUM_SAMPLES)
                .map(|i| results.iter().map(|result| result.samples[i]).sum())
                .collect();

            let overall = BenchResult::from_samples(format!("{} days", DAYS.len()), combined);
            println!("{}", overall);
        }
        Some(day) => {
            let day: usize = day.parse().unwrap();

            let start = Instant::now();
            let (part_1, part_2) = DAYS[day - 1].solve();
            println!(
                "Part 1 = {}, part 2 = {} in {:?}",
                part_1,
                part_2,
                start.elapsed()
            );
        }
    }
}

impl fmt::Display for BenchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<7} in {:.3} ± {:.3} ms",
            self.label,
            self.mean.as_secs_f64() * 1e3,
            self.std.as_secs_f64() * 1e3
        )
    }
}