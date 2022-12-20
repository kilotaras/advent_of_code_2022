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

fn advance_next(prev: Point, next: Point) -> Point {
    let (dx, dy) = step_one(prev.x - next.x, prev.y - next.y);
    Point {
        x: next.x + dx,
        y: next.y + dy,
    }
}


fn dump(rope: &[Point]) {
    const size: i32 = 5;
    const min_x: i32 = -size;
    const max_x: i32 = size;
    const min_y: i32 = -size;
    const max_y: i32 = size;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut found = false;
            for (i, point) in rope.iter().enumerate() {
                if point.x == x && point.y == y {
                    print!("{}", i);
                    found = true;
                    break;
                }
            }
            if !found {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let mut rope = vec![Point { x: 0, y: 0 }; 10];
    let mut visited = HashSet::new();
    visited.insert(rope.last().unwrap().clone());

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

        for _ in 0..distance {
            rope[0].x += dx;
            rope[0].y += dy;


            for i in 1..rope.len() {
                rope[i] = advance_next(rope[i - 1], rope[i]);
            }
            visited.insert(rope.last().unwrap().clone());
        }
    }

    let answer = visited.len();

    println!("{}", answer);
}
