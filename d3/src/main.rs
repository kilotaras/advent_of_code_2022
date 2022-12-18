use std::collections::HashSet;
use std::io::{self, BufRead};

fn get_duplicate_item(line1: &str, line2: &str, line3: &str) -> char {
    let mut first_set: HashSet<char> = line1.chars().collect();

    first_set.retain(|&c| line2.contains(c) && line3.contains(c));
    first_set.iter().next().unwrap().clone()
}

#[test]
fn test_get_duplicate_item() {
    assert_eq!(get_duplicate_item("ab", "ac", "ae"), 'a');
    assert_eq!(get_duplicate_item("xav", "bqx", "plx"), 'x');
}

fn get_score(c: char) -> i32 {
    if c.is_uppercase() {
        (c as i32) - ('A' as i32) + 27
    } else {
        (c as i32) - ('a' as i32) + 1
    }
}

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let line1 = &lines[i];
        let line2 = &lines[i + 1];
        let line3 = &lines[i + 2];
        let duplicate_item = get_duplicate_item(line1, line2, line3);
        sum += get_score(duplicate_item);
    }

    println!("{}", sum);
}
