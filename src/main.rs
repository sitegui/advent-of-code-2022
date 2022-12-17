mod data;
mod iter_utils;
mod parser;

use crate::data::Data;
use std::env::args;
use std::fmt;
use std::time::{Duration, Instant};

const NUM_WARMING: usize = 5;
const NUM_SAMPLES: usize = 15;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DayOutput {
    Int(i64, i64),
    IntStr(i64, String),
    Str(String, String),
}

struct Day {
    label: &'static str,
    solve_fn: fn(&Data) -> DayOutput,
    expected: DayOutput,
}

#[derive(Clone)]
struct BenchResult {
    label: String,
    samples: Vec<Duration>,
    mean: Duration,
    std: Duration,
}

impl Day {
    fn solve(&self, data: &Data) -> DayOutput {
        (self.solve_fn)(data)
    }

    fn bench_solve(&self, data: &Data) -> BenchResult {
        for _ in 0..NUM_WARMING {
            self.assert_solve(data);
        }

        let mut samples = Vec::with_capacity(10);
        for _ in 0..NUM_SAMPLES {
            let start = Instant::now();
            self.assert_solve(data);
            samples.push(start.elapsed());
        }

        BenchResult::from_samples(self.label, samples)
    }

    fn assert_solve(&self, data: &Data) {
        let answer = self.solve(data);
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
    ($($day:ident = $expected:expr),* $(,)?) => {
        $(mod $day;)*

        fn days() -> Vec<Day> {
            vec![
                $(Day {
                    label: stringify!($day),
                    solve_fn: $day::solve,
                    expected: $expected.into(),
                }),*
            ]
        }
    };
}

days! {
    day_1 = (68467, 203420),
    day_2 = (15422, 15442),
    day_3 = (7568, 2780),
    day_4 = (542, 900),
    day_5 = ("JDTMRWCQJ", "VHJDDCWRD"),
    day_6 = (1134, 2263),
    day_7 = (1723892, 8474158),
    day_8 = (1679, 536625),
    day_9 = (6745, 2793),
    day_10 = (15680, "ZFBFHGUP"),
    day_11 = (67830, 15305381442),
    day_12 = (408, 399),
    day_13 = (5882, 24948),
    day_14 = (1513, 22646),
}

fn main() {
    let days = days();

    match args().nth(1) {
        None => {
            println!("Will execute all days to time their individual and total execution times");

            let mut results = Vec::with_capacity(days.len());
            for day in &days {
                let data = Data::read(day.label).unwrap();
                let result = day.bench_solve(&data);
                println!("{}", result);
                results.push(result);
            }

            let combined: Vec<Duration> = (0..NUM_SAMPLES)
                .map(|i| results.iter().map(|result| result.samples[i]).sum())
                .collect();

            let overall = BenchResult::from_samples(format!("{} days", days.len()), combined);
            println!("{}", overall);
        }
        Some(day) => {
            let day = &days[day.parse::<usize>().unwrap() - 1];

            if let Ok(data) = Data::read(&format!("example_{}", day.label)) {
                let start = Instant::now();
                let answer = day.solve(&data);
                println!("Example answer {:?} in {:?}", answer, start.elapsed());
            }

            let data = Data::read(day.label).unwrap();
            let start = Instant::now();
            let answer = day.solve(&data);
            println!("Answer {:?} in {:?}", answer, start.elapsed());
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

impl From<(i64, i64)> for DayOutput {
    fn from(value: (i64, i64)) -> Self {
        DayOutput::Int(value.0, value.1)
    }
}

impl From<(i64, &str)> for DayOutput {
    fn from(value: (i64, &str)) -> Self {
        DayOutput::IntStr(value.0, value.1.to_string())
    }
}

impl From<(&str, &str)> for DayOutput {
    fn from(value: (&str, &str)) -> Self {
        DayOutput::Str(value.0.to_string(), value.1.to_string())
    }
}

impl From<(String, String)> for DayOutput {
    fn from(value: (String, String)) -> Self {
        DayOutput::Str(value.0, value.1)
    }
}

impl From<(i64, String)> for DayOutput {
    fn from(value: (i64, String)) -> Self {
        DayOutput::IntStr(value.0, value.1)
    }
}
