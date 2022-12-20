use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn step_one(dx: i32, dy: i32) -> (i32, i32) {
    fn step_one_impl(dx: i32, dy: i32) -> (i32, i32) {
        if dx > 0 && dy > 0 && (dx > 1 || dy > 1) {
            (1, 1)
        } else if dx > 1 {
            (1, 0)
        } else if dy > 1 {
            (0, 1)
        } else {
            (0, 0)
        }
    }

    let (mut dx, flip_x) = if dx < 0 { (-dx, true) } else { (dx, false) };

    let (mut dy, flip_y) = if dy < 0 { (-dy, true) } else { (dy, false) };

    (dx, dy) = step_one_impl(dx, dy);

    if flip_x {
        dx = -dx;
    }

    if flip_y {
        dy = -dy;
    }

    (dx, dy)
}

fn main() {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();

        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<i32>().unwrap();

        let (dx, dy) = match direction {
            "U" => (1, 0),
            "D" => (-1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => panic!("Unknown direction {}", direction),
        };

        for i in 0..distance {
            head.x += dx;
            head.y += dy;

            let (dx, dy) = step_one(head.x - tail.x, head.y - tail.y);
            tail.x += dx;
            tail.y += dy;
            visited.insert(tail);
        }
    }

    let answer = visited.len();

    println!("{}", answer);
}
