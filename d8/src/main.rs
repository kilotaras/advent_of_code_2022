use std::io::BufRead;
use ndarray::prelude::*;
use ndarray::Array2;
use ndarray::*;

fn solve_one_dim(view: &ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>) -> Vec<bool> {
    let mut max = view[0];
    let mut answer = vec![false; view.len()];
    for idx in 0..view.len() {
        if view[idx] > max {
            max = view[idx];
            answer[idx] = true;
        }
    }

    answer[0] = true;

    answer
}

fn main() {
    let string_to_vec_of_ints = |line: String| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
    };

    let field = {
        let field = std::io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .map(string_to_vec_of_ints)
            .collect::<Vec<_>>();
        let mut arr = Array2::zeros((field.len(), field[0].len()));
        for (idx, mut row) in arr.rows_mut().into_iter().enumerate() {
            for (jdx, col) in row.iter_mut().enumerate() {
                *col = field[idx][jdx];
            }
        }
        arr
    };

    let mut visible = field.map(|_| false);

    Zip::from(field.rows())
        .and(visible.rows_mut())
        .for_each(|field_view, mut visible_view| {
            let answer = solve_one_dim(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx] |= val;
            }

            let field_view = field_view.slice(s![..;-1]);
            let mut visible_view = visible_view.slice_mut(s![..;-1]);
            let answer = solve_one_dim(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx] |= val;
            }
        });

    Zip::from(field.columns())
        .and(visible.columns_mut())
        .for_each(|field_view, mut visible_view| {
            let answer = solve_one_dim(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx] |= val;
            }

            let field_view = field_view.slice(s![..;-1]);
            let mut visible_view = visible_view.slice_mut(s![..;-1]);
            let answer = solve_one_dim(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx] |= val;
            }
        });

    let total_visible = visible
        .iter()
        .filter(|&&val| val)
        .count();

    println!("{}", total_visible);
}
