use std::io::BufRead;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn neighbors(&self) -> Vec<Cube> {
        let mut neighbors = Vec::new();
        let left: i32 = -1;
        let right: i32 = 1;
        for dx in left..=right {
            for dy in left..=right {
                for dz in left..=right {
                    if (dx.abs() + dy.abs() + dz.abs()) != 1 {
                        continue;
                    }
                    neighbors.push(Cube {
                        x: self.x + dx,
                        y: self.y + dy,
                        z: self.z + dz,
                    });
                }
            }
        }
        neighbors
    }
}

fn parse_cube(line: &str) -> Cube {
    let mut parts = line.split(",");
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    let z = parts.next().unwrap().parse().unwrap();
    Cube { x, y, z }
}

fn main() {
    let cubes: Vec<Cube> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_cube(&line.unwrap()))
        .collect();

    let cubes_set: HashSet<Cube> = cubes.iter().cloned().collect();

    let answer = cubes
        .iter()
        .flat_map(|cube| cube.neighbors())
        .filter(|cube| !cubes_set.contains(cube))
        .count();

    println!("P1: {}", answer);

    let min_x = cubes.iter().map(|c| c.x).min().unwrap() - 1;
    let max_x = cubes.iter().map(|c| c.x).max().unwrap() + 3;

    let min_y = cubes.iter().map(|c| c.y).min().unwrap() -1;
    let max_y = cubes.iter().map(|c| c.y).max().unwrap() + 3;

    let min_z = cubes.iter().map(|c| c.z).min().unwrap() - 1;
    let max_z = cubes.iter().map(|c| c.z).max().unwrap() + 3;

    let mut visited = HashSet::new();

    let x_range = min_x..max_x;
    let y_range = min_y..max_y;
    let z_range = min_z..max_z;

    let mut queue: VecDeque<Cube> = VecDeque::new();
    queue.push_back(Cube { x: min_x, y: min_y, z: min_z });

    while !queue.is_empty() {
        let front = queue.pop_front().unwrap();

        if !x_range.contains(&front.x) || !y_range.contains(&front.y) || !z_range.contains(&front.z) {
            continue;
        }

        for neighbor in front.neighbors() {
            if visited.contains(&neighbor) {
                continue;
            }
            if cubes_set.contains(&neighbor) {
                continue;
            }
            visited.insert(neighbor.clone());
            queue.push_back(neighbor);
        }
    }

    let answer = cubes
        .iter()
        .flat_map(|cube| cube.neighbors())
        .filter(|cube| visited.contains(cube))
        .count();

    println!("P2: {}", answer);

}
