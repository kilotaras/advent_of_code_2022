use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    direction: Direction,
    row: i32,
    col: i32,
}

impl Blizzard {
    fn advanced_once(&self, field: &Field) -> Blizzard {
        let mut next_blizzard = match self.direction {
            Direction::Up => Blizzard {
                direction: Direction::Up,
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Blizzard {
                direction: Direction::Down,
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Blizzard {
                direction: Direction::Left,
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Blizzard {
                direction: Direction::Right,
                row: self.row,
                col: self.col + 1,
            },
        };

        next_blizzard.row = (next_blizzard.row + field.rows) % field.rows;
        next_blizzard.col = (next_blizzard.col + field.cols) % field.cols;

        next_blizzard
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    rows: i32,
    cols: i32,
    blizzards: HashSet<Blizzard>,
    modulo: i32,
}

impl Field {
    fn advanced(&self) -> Field {
        let new_blizzards_iter = self
            .blizzards
            .iter()
            .map(|blizzard| blizzard.advanced_once(self));
        Field {
            rows: self.rows,
            cols: self.cols,
            blizzards: new_blizzards_iter.collect(),
            modulo: self.modulo,
        }
    }

    fn contains(&self, row: i32, col: i32) -> bool {
        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let test_blizzard = Blizzard {
                direction: *direction,
                row,
                col,
            };
            if self.blizzards.contains(&test_blizzard) {
                return true;
            }
        }
        false
    }

    fn is_valid_position(&self, state: &State) -> bool {
        let row = state.row;
        let col = state.col;

        // Only start and end outside of fields are acceptable
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            let possible_values = [(-1, 0), (self.rows, self.cols - 1)];
            return possible_values.contains(&(row, col));
        }

        !self.contains(state.row, state.col)
    }

    fn start_row() -> i32 {
        -1
    }

    fn start_col() -> i32 {
        0
    }

    fn end_row(&self) -> i32 {
        self.rows
    }

    fn end_col(&self) -> i32 {
        self.cols - 1
    }

    fn is_start(&self, state: &State) -> bool {
        state.row == Field::start_row() && state.col == Field::start_col()
    }

    fn is_end(&self, state: &State) -> bool {
        state.row == self.end_row() && state.col == self.end_col()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = self.rows as usize + 2;
        let cols = self.cols as usize + 2;
        let mut field = vec![vec!['.'; cols]; rows];

        for blizzard in &self.blizzards {
            let symbol = match blizzard.direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            };
            let cur_symbol = &mut field[blizzard.row as usize + 1][blizzard.col as usize + 1];
            let next_symbol = match cur_symbol {
                '.' => symbol,
                _ => 'X',
            };

            *cur_symbol = next_symbol;
        }

        for row in 0..rows {
            field[row][0] = '#';
            field[row][cols - 1] = '#';
        }

        for col in 0..cols {
            field[0][col] = '#';
            field[rows - 1][col] = '#';
        }

        field[0][1] = '.';
        field[rows - 1][cols - 2] = '.';

        for row in field {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_field(s: &str) -> Field {
    let mut rows = 0;
    let mut cols = 0;
    let mut blizzards = HashSet::new();

    for (row, line) in s.lines().enumerate() {
        rows += 1;
        cols = line.len() as i32;

        for (col, c) in line.chars().enumerate() {
            let direction = match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => continue,
            };

            blizzards.insert(Blizzard {
                direction,
                row: (row - 1) as i32,
                col: (col - 1) as i32,
            });
        }
    }

    let rows = rows - 2;
    let cols = cols - 2;
    let field = Field {
        rows: rows,
        cols: cols,
        blizzards,
        modulo: lcm(rows, cols),
    };

    field
}

fn get_sample() -> Field {
    let input = include_str!("../sample.txt");
    let field = parse_field(input);
    field
}

fn get_input() -> Field {
    let input = include_str!("../input");
    let field = parse_field(input);
    field
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    row: i32,
    col: i32,
    offset: i32,
    phase: i32,
}

impl State {
    fn advanced_in_direction(&self, field: &Field, direction: Direction) -> Option<State> {
        let (row, col) = match direction {
            Direction::Up => (self.row - 1, self.col),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1),
            Direction::Right => (self.row, self.col + 1),
        };

        let mut next_state = State {
            row,
            col,
            offset: (self.offset + 1) % field.modulo,
            phase: self.phase,
        };

        if self.phase == 0 && field.is_end(&next_state) {
            next_state.phase = 1;
        }

        if self.phase == 1 && field.is_start(&next_state) {
            next_state.phase = 2;
        }

        if field.is_valid_position(&next_state) {
            Some(next_state)
        } else {
            None
        }
    }

    fn possible_next(&self, next_field: &Field) -> Vec<State> {
        let mut possible_next = vec![];

        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(next_state) = self.advanced_in_direction(next_field, *direction) {
                possible_next.push(next_state);
            }
        }

        // stay
        let stay_state = State {
            row: self.row,
            col: self.col,
            phase: self.phase,
            offset: (self.offset + 1) % next_field.modulo,
        };

        if next_field.is_valid_position(&stay_state) {
            possible_next.push(stay_state);
        }

        possible_next
    }
}

fn main() {
    let initial_field = if std::env::args().count() > 1 {
        get_sample()
    } else {
        get_input()
    };

    let modulo = initial_field.modulo;

    let fields = {
        let mut fields = vec![initial_field.clone()];

        for _ in 0..modulo {
            let field = fields.last().unwrap().advanced();
            fields.push(field);
        }

        fields
    };

    let initial_state = State {
        row: -1,
        col: 0,
        offset: 0,
        phase: 0,
    };

    let mut results: HashMap<State, i32> = HashMap::from([(initial_state, 0)]);
    let mut queue = VecDeque::from([initial_state]);

    // implements BFS
    while let Some(state) = queue.pop_front() {
        let current_result = results[&state];

        let next_modulo = (state.offset + 1) % modulo;
        let next_field = &fields[next_modulo as usize];
        let next_result = current_result + 1;

        let next_states = state.possible_next(&next_field);

        for next_state in next_states {
            if results.contains_key(&next_state) {
                continue;
            }
            results.insert(next_state, next_result);
            queue.push_back(next_state);
        }
    }

    let p1_answer = (0..modulo)
        .into_iter()
        .map(|offset| State {
            row: initial_field.rows,
            col: initial_field.cols - 1,
            phase: 1,
            offset,
        })
        .map(|state| results[&state])
        .min()
        .unwrap();

    println!("P1: {}", p1_answer);

    let p2_answer = (0..modulo)
        .into_iter()
        .map(|offset| State {
            row: initial_field.rows,
            col: initial_field.cols - 1,
            phase: 2,
            offset,
        })
        .map(|state| results[&state])
        .min()
        .unwrap();

    println!("P2: {}", p2_answer);

}
