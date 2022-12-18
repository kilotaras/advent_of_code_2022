use std::io::{self, BufRead};

#[derive(Debug)]
struct ElfRange {
    start: i32,
    end: i32,
}
fn parse_range(s: &str) -> ElfRange {
    let mut parts = s.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    ElfRange { start, end }
}

// Parses a pair of ranges separated by a comma.
fn parse_line(s: &str) -> (ElfRange, ElfRange) {
    let mut parts = s.split(',');
    let a = parse_range(parts.next().unwrap());
    let b = parse_range(parts.next().unwrap());
    (a, b)
}

fn contains(a: &ElfRange, b: &ElfRange) -> bool {
    fn contains_impl(a: &ElfRange, b: &ElfRange) -> bool {
        a.start <= b.start && a.end >= b.end
    }
    return contains_impl(a, b) || contains_impl(b, a);
}

fn intersect(a: &ElfRange, b: &ElfRange) -> bool {
    fn intersect_impl(a: &ElfRange, b: &ElfRange) -> bool {
        a.start <= b.start && a.end >= b.start
    }
    return intersect_impl(a, b) || intersect_impl(b, a);
}

fn main() {
    // reads all lines  and parses them
    let answer = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_line(&line))
        .filter(|(a, b)| intersect(&a, &b))
        .count();

    println!("{}", answer);
}
