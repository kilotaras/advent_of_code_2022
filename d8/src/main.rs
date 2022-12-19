use std::io::BufRead;
use ndarray::prelude::*;
use ndarray::Array2;
use ndarray::*;

fn solve_one_dim_p1(view: &ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>) -> Vec<bool> {
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

// returns number of visible trees to the left
fn solve_one_dim_p2(view: &ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>) -> Vec<usize> {
    let mut answer = vec![0; view.len()];
    for idx in 1..view.len() {
        let mut count = 0;
        for jdx in (0..idx).rev() {
            count += 1;
            if view[jdx] >= view[idx] {
                break;
            }
        }
        answer[idx] = count;
    }
    answer
}

#[test]
fn test_solve_one_dim_p2() {
    let arr = array![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let answer = solve_one_dim_p2(&arr.view());
    assert_eq!(answer, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

    let arr = array![9, 8, 7, 6, 5, 4, 3, 2, 1];
    let answer = solve_one_dim_p2(&arr.view());
    assert_eq!(answer, vec![0, 1, 1, 1, 1, 1, 1, 1, 1]);

    let arr = array![1, 2, 3, 1, 2, 5, 1];
    let answer = solve_one_dim_p2(&arr.view());
    assert_eq!(answer, vec![0, 1, 2, 1, 2, 5, 1]);
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

    let mut visible = field.map(|_| (0, 0, 0, 0));

    Zip::from(field.rows())
        .and(visible.rows_mut())
        .for_each(|field_view, mut visible_view| {
            let answer = solve_one_dim_p2(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx].0 = val;
            }

            let field_view = field_view.slice(s![..;-1]);
            let mut visible_view = visible_view.slice_mut(s![..;-1]);
            let answer = solve_one_dim_p2(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx].1 = val;
            }
        });

    Zip::from(field.columns())
        .and(visible.columns_mut())
        .for_each(|field_view, mut visible_view| {
            let answer = solve_one_dim_p2(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx].2 = val;
            }

            let field_view = field_view.slice(s![..;-1]);
            let mut visible_view = visible_view.slice_mut(s![..;-1]);
            let answer = solve_one_dim_p2(&field_view);
            for (idx, &val) in answer.iter().enumerate() {
                visible_view[idx].3 = val;
            }
        });

    let answer = visible
        .iter()
        .map(|&(a, b, c, d)| a*b*c*d)
        .max().unwrap();

    println!("{}", answer);
}
