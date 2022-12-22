#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldType {
    Empty,
    Wall,
    ForceField,
}

type Field = Vec<Vec<FieldType>>;

#[derive(Debug)]
enum Move {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

type Moves = Vec<Move>;

fn print_field(field: &Field) {
    for row in field {
        for field_type in row {
            match field_type {
                FieldType::Empty => print!("."),
                FieldType::Wall => print!("#"),
                FieldType::ForceField => print!(" "),
            }
        }
        println!("");
    }
}

fn parse_input(s: &str)->(Field, Moves) {
    let lines = s.lines().collect::<Vec<&str>>();

    let moves = {
        let move_line = lines[lines.len() - 1];
        let mut iter = move_line.chars().peekable();
        let mut moves: Moves = Vec::new();
        while iter.peek() != None {
            let c = iter.next().unwrap();
            match c {
                'L' => moves.push(Move::TurnLeft),
                'R' => moves.push(Move::TurnRight),
                _ => {
                    let mut num = String::new();
                    num.push(c);
                    while iter.peek() != None && iter.peek().unwrap().is_numeric() {
                        num.push(iter.next().unwrap());
                    }
                    moves.push(Move::Forward(num.parse::<usize>().unwrap()));
                }
            };
        }
        moves
    };

    let field = {
        let mut field = Vec::new();
        for line in lines.iter().take(lines.len() - 2) {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => row.push(FieldType::Empty),
                    '#' => row.push(FieldType::Wall),
                    ' ' => row.push(FieldType::ForceField),
                    _ => panic!("Unknown field type"),
                }
            }
            field.push(row);
        }
        let maxl = field.iter().map(|x| x.len()).max().unwrap() + 1;

        for row in field.iter_mut() {
            while row.len() < maxl {
                row.push(FieldType::ForceField);
            }
        }

        field
    };

    (field, moves)
}

fn get_sample() -> (Field, Moves) {
    let (field, moves) = parse_input(include_str!("../sample.txt"));

    (field, moves)
}

fn get_task() -> (Field, Moves) {
    let (field, moves) = parse_input(include_str!("../input"));

    (field, moves)
}

fn truemod(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn next_pos(pos: (usize, usize), dir: (i32, i32), field: &Field) -> (usize, usize) {
    let (row, col) = pos;
    let (row_dir, col_dir) = dir;

    let rows = field.len() as i32;
    let cols = field[0].len() as i32;

    let mut next_row = truemod(row as i32 + row_dir, rows);
    let mut next_col = truemod(col as i32 + col_dir, cols);


    // if we hit a force field, wrap to the other side
    if field[next_row as usize][next_col as usize] == FieldType::ForceField {
        if row_dir == 1 {
            next_row = 0;
        } else if row_dir == -1 {
            next_row = rows - 1;
        } else if col_dir == 1 {
            next_col = 0;
        } else if col_dir == -1 {
            next_col = cols - 1;
        }
    }

    while field[next_row as usize][next_col as usize] == FieldType::ForceField {
        next_row += row_dir;
        next_col += col_dir;
    };

    (next_row as usize, next_col as usize)
}

fn main() {
    let (field, moves) = get_task();
    let start_row: usize = 0;
    let start_col = field[0].iter().position(|x| *x == FieldType::Empty).unwrap();

    let directions = [
        (0, 1), // right
        (1, 0), // down
        (0, -1), // left
        (-1, 0), // up
    ];

    let mut pos = (start_row, start_col);
    let mut dir = 0;


    for m in moves {
        match m {
            Move::Forward(n) => {
                for _ in 0..n {
                    let npos = next_pos(pos, directions[dir], &field);
                    if field[npos.0][npos.1] == FieldType::Wall {
                        break;
                    }
                    pos = npos;
                }
            },
            Move::TurnLeft => {
                dir = truemod(dir as i32 - 1, directions.len() as i32) as usize;
            },
            Move::TurnRight => {
                dir = truemod(dir as i32 + 1, directions.len() as i32) as usize;
            },
        }
    }

    println!("{:?} {}", pos, dir);
    let answer = (pos.0 as i32 + 1)*1000 + (pos.1 as i32 + 1) * 4 + dir as i32;
    println!("Answer: {}", answer);
}
