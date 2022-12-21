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

    let mut air_iter = air.iter().cycle().map(|c| match c {
        '>' => 1,
        '<' => -1,
        _ => panic!("Invalid air {:?}", c),
    });

    let mut figure_iter = figures.iter().cycle();

    for num in 0..2022 {
        let figure = figure_iter.next().unwrap();
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
            let dcol = air_iter.next().unwrap();
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

        // println!("S");
        // dump_field(&fields);
    }

    println!("{}", top_row);
}
