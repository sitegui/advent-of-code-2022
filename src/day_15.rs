use crate::data::Data;
use crate::nom_parser::*;
use crate::xy::XY;
use crate::DayOutput;
use itertools::Itertools;
use std::cmp::Reverse;

#[derive(Debug)]
struct Sensor {
    xy: XY,
    beacon: XY,
    radius: i32,
}

#[derive(Debug)]
struct LineCovering {
    y: i32,
    x_intervals: Vec<(i32, i32)>,
}

pub fn solve(data: &Data) -> DayOutput {
    let mut sensors = lines(parse_sensor).consume_all(data.bytes());

    let target_y = if data.is_example() { 10 } else { 2000000 };
    let mut covered = LineCovering::new(target_y, sensors.len());

    for sensor in &sensors {
        covered.add_intersection(sensor);
    }

    let mut part_1 = 0;
    for &(start, end) in &covered.x_intervals {
        part_1 += end - start + 1;
    }

    let beacons_at_target_y = sensors
        .iter()
        .map(|sensor| sensor.beacon)
        .filter(|beacon| beacon.y == target_y)
        .unique();
    for beacon in beacons_at_target_y {
        if covered
            .x_intervals
            .iter()
            .any(|&(start, end)| beacon.x >= start && beacon.y <= end)
        {
            part_1 -= 1;
        }
    }

    let search_space_size = if data.is_example() { 20 } else { 4000000 };

    // Sort sensors by how "inside" the search space they are
    sensors.sort_by_key(|sensor| {
        let min_x = sensor.xy.x - sensor.radius;
        let max_x = sensor.xy.x + sensor.radius;
        let min_y = sensor.xy.y - sensor.radius;
        let max_y = sensor.xy.y + sensor.radius;

        let x_start = min_x.max(0) as i64;
        let x_end = max_x.min(search_space_size) as i64;
        let y_start = min_y.max(0) as i64;
        let y_end = max_y.min(search_space_size) as i64;

        let intersection = if x_end >= x_start && y_end >= y_start {
            (x_end - x_start + 1) * (y_end - y_start + 1)
        } else {
            0
        };

        Reverse(intersection)
    });

    let mut non_fully_covered_lines = (0..=search_space_size)
        .map(|y| LineCovering::new(y, sensors.len()))
        .collect_vec();
    for sensor in &sensors {
        non_fully_covered_lines.retain_mut(|line_covering| {
            line_covering.add_intersection(sensor);
            !line_covering.covers(0, search_space_size)
        });
    }
    assert_eq!(non_fully_covered_lines.len(), 1);
    let single_line = non_fully_covered_lines.into_iter().next().unwrap();
    let y = single_line.y;
    assert_eq!(single_line.x_intervals.len(), 2);
    assert_eq!(
        single_line.x_intervals[0].1 + 2,
        single_line.x_intervals[1].0
    );
    let x = single_line.x_intervals[0].1 + 1;
    let part_2 = x as i64 * 4000000 + y as i64;

    (part_1 as i64, part_2).into()
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

impl LineCovering {
    fn new(y: i32, max_intervals: usize) -> Self {
        LineCovering {
            y,
            x_intervals: Vec::with_capacity(max_intervals),
        }
    }

    fn add_intersection(&mut self, sensor: &Sensor) {
        let sensor_to_target = (sensor.xy.y - self.y).abs();
        let half_width = sensor.radius - sensor_to_target;

        if half_width > 0 {
            let start = sensor.xy.x - half_width;
            let end = sensor.xy.x + half_width;
            let index = match self
                .x_intervals
                .binary_search_by_key(&start, |interval| interval.0)
            {
                Ok(i) | Err(i) => i,
            };
            self.x_intervals.insert(index, (start, end));
            self.merge_intervals();
        }
    }

    fn merge_intervals(&mut self) {
        if self.x_intervals.len() < 2 {
            return;
        }

        let mut new_intervals = Vec::with_capacity(self.x_intervals.capacity());
        let mut intervals = self.x_intervals.drain(..);

        let mut current = intervals.next().unwrap();
        for (start, end) in intervals {
            if start > current.1 + 1 {
                new_intervals.push(current);
                current = (start, end);
            } else {
                current.1 = current.1.max(end);
            }
        }
        new_intervals.push(current);

        self.x_intervals = new_intervals;
    }

    fn covers(&self, start: i32, end: i32) -> bool {
        for interval in &self.x_intervals {
            if interval.0 <= start && interval.1 >= end {
                return true;
            }
        }

        false
    }
}
