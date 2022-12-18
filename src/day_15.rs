use crate::data::Data;
use crate::nom_parser::*;
use crate::xy::XY;
use crate::DayOutput;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Sensor {
    xy: XY,
    beacon: XY,
    radius: i32,
}

pub fn solve(data: &Data) -> DayOutput {
    let mut sensors = lines(parse_sensor).consume_all(data.bytes());
    sensors.sort_by_key(|sensor| Reverse(sensor.radius));
    let target_y = if data.is_example() { 10 } else { 2000000 };
    let mut covered_intervals = vec![];

    for sensor in &sensors {
        let sensor_to_target = (sensor.xy.y - target_y).abs();
        let half_width = sensor.radius - sensor_to_target;

        if half_width > 0 {
            covered_intervals.push((sensor.xy.x - half_width, sensor.xy.x + half_width));
        }
    }
    covered_intervals = merge_intervals(covered_intervals);

    let mut part_1 = 0;
    for &(start, end) in &covered_intervals {
        part_1 += end - start + 1;
    }

    let beacons_at_target_y = sensors
        .iter()
        .map(|sensor| sensor.beacon)
        .filter(|beacon| beacon.y == target_y)
        .unique();
    for beacon in beacons_at_target_y {
        if covered_intervals
            .iter()
            .any(|&(start, end)| beacon.x >= start && beacon.y <= end)
        {
            part_1 -= 1;
        }
    }

    let search_space_size = if data.is_example() { 20 } else { 4000000 };
    let mut non_fully_covered_xs = (0..=search_space_size).collect_vec();
    for sensor in &sensors {
        non_fully_covered_xs.retain(|&x| {
            let start_distance = sensor.xy.manhattan_distance(XY::new(x, 0));
            let end_distance = sensor.xy.manhattan_distance(XY::new(x, search_space_size));

            start_distance > sensor.radius || end_distance > sensor.radius
        });
        eprintln!(
            "non_fully_covered_xs.len() = {:?}",
            non_fully_covered_xs.len()
        );
    }

    (part_1 as i64, 0).into()
}

fn parse_sensor(input: &[u8]) -> PResult<Sensor> {
    map(
        tuple((
            tag(b"Sensor at "),
            parse_xy,
            tag(b": closest beacon is at "),
            parse_xy,
        )),
        |(_, sensor, _, beacon)| Sensor {
            xy: sensor,
            beacon,
            radius: sensor.manhattan_distance(beacon),
        },
    )(input)
}

fn parse_xy(input: &[u8]) -> PResult<XY> {
    map(
        tuple((tag(b"x="), i32, tag(b", y="), i32)),
        |(_, x, _, y)| XY::new(x, y),
    )(input)
}

fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort_by_key(|&(start, _)| start);
    let mut intervals = intervals.into_iter();
    let mut result = vec![];

    let mut current = intervals.next().unwrap();
    for (start, end) in intervals {
        if start > current.1 {
            result.push(current);
            current = (start, end);
        } else {
            current.1 = current.1.max(end);
        }
    }
    result.push(current);

    result
}
