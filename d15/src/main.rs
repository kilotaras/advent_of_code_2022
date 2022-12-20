use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::BufRead;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    col: i32,
    row: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Measurement {
    sensor: Point,
    beacon: Point,
}

fn parse_message(s: &str) -> Measurement {
    let mut iter = s.split(|c| " =,:".contains(c));
    let sensor = {
        // Sensor at x
        let col = iter.nth(3).unwrap().parse().unwrap();
        // y
        let row = iter.nth(2).unwrap().parse().unwrap();
        Point { col, row }
    };

    let beacon = {
        let col = iter.nth(6).unwrap().parse().unwrap();
        let row = iter.nth(2).unwrap().parse().unwrap();
        Point { col, row }
    };

    Measurement { sensor, beacon }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let msg = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let m = parse_message(msg);
        assert_eq!(
            m,
            Measurement {
                sensor: Point { col: 2, row: 18 },
                beacon: Point { col: -2, row: 15 },
            }
        );
    }
}

fn find_position_slow(measurements: &[Measurement], row: i32) -> Vec<i32> {
    let min_col = measurements
        .iter()
        .map(|m| min(m.sensor.col, m.beacon.col))
        .min()
        .unwrap();

    let max_col = measurements
        .iter()
        .map(|m| max(m.sensor.col, m.beacon.col))
        .max()
        .unwrap();

    let d = max_col - min_col + 1;

    let d = 3 * d + row.abs() * 2 + 100;
    let left_edge = min_col - d;
    let right_edge = max_col + d;

    let blocked = |col: i32| {
        let blocked = measurements.iter().any(|m| {
            let distance_to_candidate = (m.sensor.col - col).abs() + (m.sensor.row - row).abs();
            let distance_to_beacon =
                (m.sensor.col - m.beacon.col).abs() + (m.sensor.row - m.beacon.row).abs();

            distance_to_candidate <= distance_to_beacon
        });

        let has_beacon = measurements
            .iter()
            .any(|m| m.beacon.col == col && m.beacon.row == row);

        blocked && !has_beacon
    };

    (left_edge..right_edge)
        .filter(|col| blocked(*col))
        .collect()
}

#[allow(dead_code)]
fn find_answer_slow(measurements: &[Measurement], row: i32) -> i32 {
    let position = find_position_slow(measurements, row);
    position.len() as i32
}

fn find_positions_fast(measurements: &[Measurement], row: i32) -> Vec<Range<i32>> {
    let mut ranges = Vec::new();

    for m in measurements.iter() {
        let distance_to_beacon =
            (m.sensor.col - m.beacon.col).abs() + (m.sensor.row - m.beacon.row).abs();

        let distance_to_row = (m.sensor.row - row).abs();

        if distance_to_row <= distance_to_beacon {
            let dleft = distance_to_beacon - distance_to_row;
            let left = m.sensor.col - dleft;
            let right = m.sensor.col + dleft;
            ranges.push(left..(right + 1));
        }
    }

    ranges.sort_by_key(|r| (r.start, r.end));

    if ranges.is_empty() {
        return vec![];
    }

    let mut merged: Vec<Range<i32>> = vec![ranges[0].clone()];

    for r in ranges.iter().skip(1) {
        let mut last = merged.last_mut().unwrap();
        if r.start <= last.end {
            last.end = max(r.end, last.end);
        } else {
            merged.push(r.clone());
        }
    }

    let mut beacons = Vec::new();

    for m in measurements.iter() {
        if m.beacon.row == row {
            beacons.push(m.beacon.col);
        }
    }

    beacons.sort();

    let mut valid_ranges = Vec::new();

    for range in merged {
        let mut current_range = range.clone();
        for beacon in beacons.iter() {
            if current_range.contains(&beacon) {
                let to_add = current_range.start..*beacon;
                current_range = (*beacon + 1)..current_range.end;

                valid_ranges.push(to_add);
            }
        }
        valid_ranges.push(current_range);
    }

    valid_ranges
}

#[allow(dead_code)]
fn find_answer_fast(measurements: &[Measurement], row: i32) -> i32 {
    let ranges = find_positions_fast(measurements, row);
    ranges.iter().map(|r| r.end - r.start).sum()
}
#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn test_slow() {
        let small: Vec<Measurement> = include_str!("../sample.txt")
            .lines()
            .map(|line| parse_message(line))
            .collect();

        assert_eq!(find_answer_slow(&small, 10), 26);
    }

    #[test]
    #[ignore]
    fn test_slow_large() {
        let large: Vec<Measurement> = include_str!("../input")
            .lines()
            .map(|line| parse_message(line))
            .collect();

        assert_eq!(find_answer_slow(&large, 2000000), 4582667);
    }

    #[test]
    fn test_fast_small() {
        let small: Vec<Measurement> = include_str!("../sample.txt")
            .lines()
            .map(|line| parse_message(line))
            .collect();

        assert_eq!(find_answer_fast(&small, 10), 26);
    }

    #[test]
    fn test_fast_small_compare_with_slow() {
        let small: Vec<Measurement> = include_str!("../sample.txt")
            .lines()
            .map(|line| parse_message(line))
            .collect();

        for row in 0..20 {
            assert_eq!(
                find_answer_fast(&small, row),
                find_answer_slow(&small, row),
                "Differs at row {}",
                row
            );
        }
    }

    #[test]
    fn test_fast_large() {
        let large: Vec<Measurement> = include_str!("../input")
            .lines()
            .map(|line| parse_message(line))
            .collect();

        assert_eq!(find_answer_fast(&large, 2000000), 4582667);
    }
}

fn main() {
    let measurements: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_message(&line.unwrap()))
        .collect();
}
