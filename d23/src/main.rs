use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Elf {
    row: i32,
    col: i32,
}

impl Elf {
    fn advanced(&self, row: i32, col: i32) -> Elf {
        Elf {
            row: self.row + row,
            col: self.col + col,
        }
    }

    fn advanced_in_direction(&self, direction: &Direction) -> (Elf, (Elf, Elf)) {
        match direction {
            Direction::Up => (
                self.advanced(-1, 0),
                (self.advanced(-1, 1), self.advanced(-1, -1)),
            ),
            Direction::Down => (
                self.advanced(1, 0),
                (self.advanced(1, 1), self.advanced(1, -1)),
            ),
            Direction::Left => (
                self.advanced(0, -1),
                (self.advanced(-1, -1), self.advanced(1, -1)),
            ),
            Direction::Right => (
                self.advanced(0, 1),
                (self.advanced(-1, 1), self.advanced(1, 1)),
            ),
        }
    }
}

type ElfSet = HashSet<Elf>;

fn get_sample() -> ElfSet {
    let lines = include_str!("../sample.txt").lines();

    let mut elves = ElfSet::new();
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Elf {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }

    elves
}

fn get_task() -> ElfSet {
    let lines = include_str!("../input").lines();

    let mut elves = ElfSet::new();
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Elf {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }

    elves
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_moves(elf: &Elf, positions: &ElfSet, directions: &[Direction]) -> Elf {
    let mut found = false;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let new_elf = elf.advanced(dx, dy);
            if positions.contains(&new_elf) {
                found = true;
                break;
            }
        }
    }

    if !found {
        return elf.clone();
    }

    for direction in directions {
        let (new_elf, (left, right)) = elf.advanced_in_direction(direction);
        if positions.contains(&new_elf) {
            continue;
        }

        if positions.contains(&left) || positions.contains(&right) {
            continue;
        }

        return new_elf;
    }

    elf.clone()
}

fn dump(elves: &ElfSet) {
    let min_row = elves.iter().map(|elf| elf.row).min().unwrap();
    let max_row = elves.iter().map(|elf| elf.row).max().unwrap();

    let min_col = elves.iter().map(|elf| elf.col).min().unwrap();
    let max_col = elves.iter().map(|elf| elf.col).max().unwrap();

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if elves.contains(&Elf { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let mut elves = get_task();
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    // dump(&elves);

    let mut round = 0;
    loop {
        let mut directions_iter = directions.iter().cycle().skip(round);
        let directions = [
            *directions_iter.next().unwrap(),
            *directions_iter.next().unwrap(),
            *directions_iter.next().unwrap(),
            *directions_iter.next().unwrap(),
        ];

        let proposed_positions: Vec<_> = elves
            .iter()
            .map(|elf| (elf.clone(), get_moves(elf, &elves, &directions)))
            .collect();

        // get hashmap with counts of new_positions
        let posisition_count = proposed_positions
            .iter()
            .map(|(_, new_position)| new_position)
            .fold(HashMap::new(), |mut acc, new_position| {
                *acc.entry(new_position).or_insert(0) += 1;
                acc
            });

        let mut new_elves = ElfSet::new();

        for (elf, new_position) in &proposed_positions {
            if posisition_count[new_position] == 1 {
                new_elves.insert(new_position.clone());
            } else {
                new_elves.insert(elf.clone());
            }
        }

        if new_elves == elves {
            break;
        }

        elves = new_elves;
        round += 1;
    }

    println!("Round: {}", round + 1);

    // let min_row = elves.iter().map(|elf| elf.row).min().unwrap();
    // let max_row = elves.iter().map(|elf| elf.row).max().unwrap();

    // let min_col = elves.iter().map(|elf| elf.col).min().unwrap();
    // let max_col = elves.iter().map(|elf| elf.col).max().unwrap();

    // let drow = max_row - min_row + 1;
    // let dcol = max_col - min_col + 1;

    // let answer = drow*dcol - elves.len() as i32;

    // println!("Answer: {}", answer);
}
