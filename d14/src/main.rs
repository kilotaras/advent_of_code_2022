use std::io::BufRead;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(s: &str) -> Vec<Point> {
    s.split("->")
        .map(|s| {
            let mut iter = s.trim().split(",");
            let y = iter.next().unwrap().parse().unwrap();
            let x = iter.next().unwrap().parse().unwrap();
            Point { x, y }
        })
        .collect()
}

fn main() {
    let chunks: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse_line(&x))
        .collect();
    let lines = chunks.iter().flat_map(|x| x.iter()).map(|x| x.x).max().unwrap() as usize + 5;
    let rows = chunks.iter().flat_map(|x| x.iter()).map(|x| x.y).max().unwrap() as usize + 5;

    let mut field = vec![vec!['.'; rows as usize]; lines as usize];

    for points in chunks {
        points.iter()
            .zip(points.iter().skip(1))
            .for_each(|(p1, p2)| {
                let make_range = |l, r| if l < r { l..=r } else { r..=l };
                if p1.x == p2.x {
                    for y in make_range(p1.y, p2.y) {
                        field[p1.x as usize][y as usize] = '#';
                    }
                } else {
                    for x in make_range(p1.x, p2.x) {
                        field[x as usize][p1.y as usize] = '#';
                    }
                }
            });
    }

    let mut cnt = 0;

    loop {
        let (mut px, mut py) = (0, 500);
        loop {
            if px == (lines - 1) {
                break;
            }

            let attempts: [i32; 3] = [0, -1, 1];

            let mut found = false;
            for dy in attempts {
                let nx = px + 1;
                let ny = (py + dy) as usize;
                if field[nx][ny] == '.' {
                    px = nx;
                    py = ny as i32;
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }
        }
        field[px][py as usize] = 'o';

        if px == (lines - 1) {
            break;
        }

        cnt += 1;
    }

    println!("{}", cnt)
}
