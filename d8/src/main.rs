use std::io::BufRead;

fn main() {
    let string_to_vec_of_ints = |line: String| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
    };

    let field = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(string_to_vec_of_ints)
        .collect::<Vec<_>>();

    let mut visible = field
        .iter()
        .map(|row| vec![false; row.len()])
        .collect::<Vec<_>>();

    let rows = field.len();
    let cols = field[0].len();

    for row_idx in 0..rows {
        visible[row_idx][0] = true;

        let mut max = field[row_idx][0];
        for col_idx in 0..cols {
            let height = field[row_idx][col_idx];
            if height > max {
                max = height;
                visible[row_idx][col_idx] = true;
            }
        }

        // now the other way
        visible[row_idx][cols - 1] = true;
        let mut max = field[row_idx][cols - 1];
        for col_idx in (0..cols).rev() {
            let height = field[row_idx][col_idx];
            if height > max {
                max = height;
                visible[row_idx][col_idx] = true;
            }
        }
    }

    // and now columns
    for col_idx in 0..cols {
        visible[0][col_idx] = true;

        let mut max = field[0][col_idx];
        for row_idx in 0..rows {
            let height = field[row_idx][col_idx];
            if height > max {
                max = height;
                visible[row_idx][col_idx] = true;
            }
        }

        // now the other way
        visible[rows - 1][col_idx] = true;
        let mut max = field[rows - 1][col_idx];
        for row_idx in (0..rows).rev() {
            let height = field[row_idx][col_idx];
            if height > max {
                max = height;
                visible[row_idx][col_idx] = true;
            }
        }
    }

    let total_visible = visible
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum::<usize>();

    println!("{}", total_visible);
}
