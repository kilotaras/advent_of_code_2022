use std::io::BufRead;
use std::collections::VecDeque;

fn char_to_height(c: char) -> i32 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => c as i32 - 'a' as i32,
    }
}

fn find_position(field: &Vec<Vec<char>>, needle: char) -> (usize, usize) {
    for (row, line) in field.iter().enumerate() {
        for (column, c) in line.iter().enumerate() {
            if *c == needle {
                return (row, column);
            }
        }
    }
    panic!("Could not find needle");
}


fn main() {
    let field_lines: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().collect())
        .collect();

    let field: Vec<Vec<i32>> = field_lines
        .iter()
        .map(|x| x.iter().map(|c| char_to_height(*c)).collect())
        .collect();

    let (x, y) = find_position(&field_lines, 'S');
    let (ex, ey) = find_position(&field_lines, 'E');

    let mut distance = vec![vec![-1; field[0].len()]; field.len()];
    let mut queue = VecDeque::from([(x, y )]);

    distance[x][y] = 0;

    let row_range = 0..field.len();;
    let col_range = 0..field[0].len();

    // Implement BFS here
    while let Some((row, column)) = queue.pop_front() {
        let current_distance = distance[row][column];
        let current_height = field[row][column];

        let row = row as i32;
        let column = column as i32;

        let neighbors = [
            (row - 1, column),
            (row + 1, column),
            (row, column - 1),
            (row, column + 1),
        ];

        for (nrow, ncolumn) in neighbors.iter() {
            let nrow = *nrow as usize;
            let ncolumn = *ncolumn as usize;
            if row_range.contains(&nrow) && col_range.contains(&ncolumn) {
                let neighbor_height = field[nrow][ncolumn];
                if neighbor_height <= current_height + 1 && distance[nrow][ncolumn] == -1 {
                    distance[nrow][ncolumn] = current_distance + 1;
                    queue.push_back((nrow, ncolumn));
                }
            }
        }
    }

    println!("{}", distance[ex][ey]);
}
