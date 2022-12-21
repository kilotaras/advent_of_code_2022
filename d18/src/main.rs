use std::io::BufRead;
use std::collections::HashSet;

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


    println!("{}", answer);
}
