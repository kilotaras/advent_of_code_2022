use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    drow: i32,
    dcol: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Figure {
    points: Vec<Point>,
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cols = self.points.iter().map(|p| p.dcol).max().unwrap() + 1;
        let rows = self.points.iter().map(|p| p.drow).max().unwrap() + 1;

        for row in (0..rows).rev() {
            for col in 0..cols {
                if self.points.contains(&Point {
                    drow: row,
                    dcol: col,
                }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone)]
struct Rock {
    figure: Figure,
    position: Position,
}

impl Rock {
    fn points(&self) -> Vec<Position> {
        self.figure
            .points
            .iter()
            .map(|p| Position {
                row: self.position.row + p.drow,
                col: self.position.col + p.dcol,
            })
            .collect()
    }

    fn advance(&self, drow: i32, dcol: i32) -> Rock {
        Rock {
            figure: self.figure.clone(),
            position: Position {
                row: self.position.row + drow,
                col: self.position.col + dcol,
            },
        }
    }

    fn top(&self) -> Position {
        let mut top = self.position;
        for p in self.points() {
            if p.row > top.row {
                top = p;
            }
        }
        top
    }

    fn is_valid(&self, field: &[Row]) -> bool {
        for p in self.points() {
            if p.col < 0 || p.col >= 7 {
                return false;
            }

            let row = p.row as usize;
            let col = p.col as usize;
            if row < field.len() && field[row][col] {
                return false;
            }
        }
        true
    }
}

type Row = [bool; 7];

fn build_field_string(field: &[Row]) -> Vec<String> {
    let mut answer = Vec::new();
    for row in field.iter() {
        let mut s = String::new();
        for &col in row.iter() {
            if col {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        answer.push(s);
    }
    answer
}

fn dump(field: &[Row], rock: &Rock) {
    let mut answer = build_field_string(field);
    for point in rock.points() {
        let row = point.row as usize;
        let col = point.col as usize;
        while answer.len() <= row {
            answer.push(".......".to_string());
        }
        answer[row].replace_range(col..col + 1, "@");
    }

    for row in answer.iter().rev() {
        println!("|{}|", row);
    }
}

fn dump_field(field: &[Row]) {
    let mut answer = build_field_string(field);
    for row in answer.iter().rev() {
        println!("{}", row);
    }
    println!();
}

fn main() {
    let figures = [
        Figure {
            points: vec![
                Point { drow: 0, dcol: 0 },
                Point { drow: 0, dcol: 1 },
                Point { drow: 0, dcol: 2 },
                Point { drow: 0, dcol: 3 },
            ],
        },
        Figure {
            points: vec![
                Point { drow: 0, dcol: 1 },
                Point { drow: 1, dcol: 0 },
                Point { drow: 1, dcol: 1 },
                Point { drow: 1, dcol: 2 },
                Point { drow: 2, dcol: 1 },
            ],
        },
        Figure {
            points: vec![
                Point { drow: 0, dcol: 0 },
                Point { drow: 0, dcol: 1 },
                Point { drow: 0, dcol: 2 },
                Point { drow: 1, dcol: 2 },
                Point { drow: 2, dcol: 2 },
            ],
        },
        Figure {
            points: vec![
                Point { drow: 0, dcol: 0 },
                Point { drow: 1, dcol: 0 },
                Point { drow: 2, dcol: 0 },
                Point { drow: 3, dcol: 0 },
            ],
        },
        Figure {
            points: vec![
                Point { drow: 0, dcol: 0 },
                Point { drow: 0, dcol: 1 },
                Point { drow: 1, dcol: 0 },
                Point { drow: 1, dcol: 1 },
            ],
        },
    ];

    let empty_row = [false; 7];
    let mut fields: Vec<Row> = vec![[true; 7]];

    let mut top_row = 0;

    let air = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
        .chars()
        .collect::<Vec<_>>();

    let air = include_str!("../input")
        .chars()
        .filter(|c| *c == '>' || *c == '<')
        .collect::<Vec<_>>();

    let mut air_index = 0;
    let mut figure_index = 0;

    let lcm = air.len() * figures.len();
    let max_v = lcm * 1000;

    let mut state = Vec::new();
    for num in 0..max_v {
        if num % 100000 == 0 {
            println!("{:7}/{}", num, max_v);
        }
        let start_fig = figure_index;
        let start_air = air_index;
        let start_top = top_row;

        let figure = &figures[figure_index];
        figure_index = (figure_index + 1) % figures.len();
        let mut rock = Rock {
            figure: figure.clone(),
            position: Position {
                row: top_row + 4,
                col: 2,
            },
        };

        // println!("N");
        // dump(&fields, &rock);

        loop {
            let dcol = if air[air_index] == '>' { 1 } else { -1 };
            air_index = (air_index + 1) % air.len();

            let new_rock = rock.advance(0, dcol);

            // println!("{}", if dcol > 0 { '>' } else { '<' });
            if new_rock.is_valid(&fields) {
                rock = new_rock;
                // dump(&fields, &rock);
            }

            // println!("D");
            let new_rock = rock.advance(-1, 0);
            if new_rock.is_valid(&fields) {
                rock = new_rock;
                // dump(&fields, &rock);
            } else {
                break;
            }
        }

        top_row = std::cmp::max(rock.top().row, top_row);

        while fields.len() <= (top_row + 1) as usize {
            fields.push(empty_row);
        }

        for p in rock.points() {
            fields[p.row as usize][p.col as usize] = true;
        }

        let top_advance = top_row - start_top;
        let top_distance = top_row - rock.top().row;

        state.push((start_fig, start_air, top_advance, top_distance));
    }

    println!("{}", top_row);

    let mut distance = 0;

    let cycle_start = 3 * lcm;
    for cycle_length in 1.. {
        let cycle_length = cycle_length * lcm;

        let bad = (cycle_start..(cycle_start + cycle_length))
            .any(|p| state[p] != state[p + cycle_length]);

        if !bad {
            distance = cycle_length;
            break;
        }
    }

    let mut top_row = 0;
    for (_, _, top_advance, _) in &state[0..cycle_start] {
        top_row += (*top_advance) as i64;
    }

    let mut cycle_increase = 0;
    for (_, _, top_advance, _) in &state[cycle_start..(cycle_start + distance)] {
        cycle_increase += (*top_advance) as i64;
    }

    let target_row: i64 = 1000000000000;
    let target_row = target_row - cycle_start as i64;

    let cycles = target_row / distance as i64;
    let rest = target_row % distance as i64;

    top_row += cycles * cycle_increase;

    for (_, _, top_advance, _) in &state[cycle_start..(cycle_start + rest as usize)] {
        top_row += (*top_advance) as i64;
    }

    println!("{}", top_row);
}
