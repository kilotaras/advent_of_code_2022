use std::cmp::{max, min};
use std::io::BufRead;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
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
        let x = iter.nth(3).unwrap().parse().unwrap();
        // y
        let y = iter.nth(2).unwrap().parse().unwrap();
        Point { x, y }
    };

    let beacon = {
        let x = iter.nth(6).unwrap().parse().unwrap();
        let y = iter.nth(2).unwrap().parse().unwrap();
        Point { x, y }
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
                sensor: Point { x: 2, y: 18 },
                beacon: Point { x: -2, y: 15 },
            }
        );
    }
}

fn find_answer_slow(measurements: &[Measurement], row: i32) -> i32 {
    let min_y = measurements
        .iter()
        .map(|m| min(m.sensor.y, m.beacon.y))
        .min()
        .unwrap();

    let max_y = measurements
        .iter()
        .map(|m| max(m.sensor.y, m.beacon.y))
        .max()
        .unwrap();


    let d = max_y - min_y + 1;

    let left_edge = min_y - 3 * d - 2 * row;
    let right_edge = max_y + 3 * d + 2 * row;

    let blocked = |col: i32| {
        let blocked = measurements.iter().any(|m| {
            let distance_to_candidate = (m.sensor.x - col).abs() + (m.sensor.y - row).abs();
            let distance_to_beacon =
                (m.sensor.x - m.beacon.x).abs() + (m.sensor.y - m.beacon.y).abs();

            distance_to_candidate <= distance_to_beacon
        });

        let has_beacon = measurements
            .iter()
            .any(|m| m.beacon.x == col && m.beacon.y == row);

        return blocked && !has_beacon;
    };

    (left_edge..right_edge).filter(|col| blocked(*col)).count() as i32
}

fn main() {
    let measurements: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_message(&line.unwrap()))
        .collect();

    let answer = find_answer_slow(&measurements, 2000000);
    println!("{}", answer);
}
