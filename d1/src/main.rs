use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read");

    let mut elves = input
        .split("\n\n")
        .map(str::trim)
        .map(|elf| {
            elf.split('\n').map(|f| f.parse::<i32>().unwrap()).sum()
        })
        .collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(a));

    // Get sum of first 3 elements
    let result = elves.iter().take(3).sum::<i32>();

    println!("{}", result);
}
