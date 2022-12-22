fn get_sample() -> Vec<i64> {
    include_str!("../sample.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn get_task() -> Vec<i64> {
    include_str!("../input")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn true_mod(v: i64, m: i32) -> i32 {
    let rem = (v % (m as i64)) as i32;
    if rem < 0 {
        rem + m
    } else {
        rem
    }
}

fn move_simple(buffer: &mut Vec<i32>, from: i32, to: i32) {
    let value = buffer[from as usize];
    buffer.remove(from as usize);
    buffer.insert(to as usize, value);
}

fn move_once(buffer: &mut Vec<i32>, pos: i32, value: i64) {
    let len = buffer.len() as i32;
    let mut pos = pos;

    let module = (len - 1)*(len - 1);
    let mut value = true_mod(value, module);


    loop {
        let new_pos = pos + value;
        if (0..len).contains(&new_pos) {
            // no wrap arounds, all good
            move_simple(buffer, pos, new_pos);
            break;
        }

        if new_pos < 0 {
            assert!(value < 0);
            move_simple(buffer, pos, 0);
            value += pos;
            move_simple(buffer, 0, len - 2);
            value += 1;
            pos = len - 2;
        }

        if new_pos >= len {
            assert!(value > 0);
            move_simple(buffer, pos, len - 1);
            value -= len - 1 - pos;
            move_simple(buffer, len - 1, 1);
            value -= 1;
            pos = 1;
        }
    }
}

fn advance(input: &[i64], positions: &mut Vec<i32>, turn: i32) {
    let mut pos = positions.iter().position(|&x| x == turn).unwrap();
    let value = input[positions[pos] as usize];
    move_once(positions, pos as i32, value);
}

fn make_answer(positions: &[i32], input: &[i64]) -> Vec<i64> {
    let mut answer = vec![];
    for pos in positions {
        answer.push(input[*pos as usize]);
    }
    answer
}

// #[test]
// fn test_sample() {
//     let input = get_sample();

//     let len = input.len() as i32;
//     let mut positions: Vec<i32> = (0..len).collect();

//     let states = [
//         [1, 2, -3, 3, -2, 0, 4],
//         [2, 1, -3, 3, -2, 0, 4],
//         [1, -3, 2, 3, -2, 0, 4],
//         [1, 2, 3, -2, -3, 0, 4],
//         [1, 2, -2, -3, 0, 3, 4],
//         [1, 2, -3, 0, 3, 4, -2],
//         [1, 2, -3, 0, 3, 4, -2],
//         [1, 2, -3, 4, 0, 3, -2],
//     ];


//     for v in 0..len {
//         let current = make_answer(&positions, &input);

//         for val in &current {
//             print!("{}, ", val);
//         }
//         println!();
//         for val in states[v as usize] {
//             print!("{}, ", val);
//         }
//         println!();
//         println!();

//         assert_eq!(current, states[v as usize]);
//         advance(&input, &mut positions, v);
//     }
// }

fn main() {
    let input = get_task();

    let len = input.len() as i32;
    let mut positions: Vec<i32> = (0..len).collect();

    for p in 0..len {
        advance(&input, &mut positions, p);
    }

    let result = make_answer(&positions, &input);
    let zero_pos = result.iter().position(|&x| x == 0).unwrap();

    let mut p1_answer = 0;
    for i in 1..4 {
        let ipos = true_mod((zero_pos as i32 + i*1000) as i64, len) as usize;
        println!("{}: {}", i, result[ipos]);
        p1_answer += result[ipos];
    }

    println!("P1: {}", p1_answer);

    let input: Vec<i64> = input.iter().map(|v| v * 811589153).collect();


    let mut positions: Vec<i32> = (0..len).collect();

    for round in 0..10 {
        for p in 0..len {
            if p%100 == 0 {
                println!("{:4}/{}", p, round);
            }
            advance(&input, &mut positions, p);
        }
    }

    let result = make_answer(&positions, &input);
    let zero_pos = result.iter().position(|&x| x == 0).unwrap();

    let mut p2_answer = 0;
    for i in 1..4 {
        let ipos = true_mod((zero_pos as i32 + i*1000) as i64, len) as usize;
        println!("{}: {}", i, result[ipos]);
        p2_answer += result[ipos];
    }

    println!("P1: {}", p1_answer);
    println!("P2: {}", p2_answer);
}
