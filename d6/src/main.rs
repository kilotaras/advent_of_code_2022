use std::collections::HashSet;

fn solve_puzzle(line: &str, cnt: usize) -> usize {
    let chars = line.chars().collect::<Vec<char>>();
    for i in (cnt - 1)..chars.len() {
        // Gets cnt chars that end and i
        let message = chars.iter().skip(i - (cnt - 1)).take(cnt).collect::<HashSet<&char>>();
        if message.len() == cnt {
            return i + 1;
        }
    }
    panic!("No solution found")
}

#[test]
fn test_part1_samples() {
    let samples = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];
    for (line, expected) in samples.iter() {
        assert_eq!(solve_puzzle(line, 4), *expected);
    }
}

#[test]
fn test_part2_samples() {
    let samples = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];
    for (line, expected) in samples.iter() {
        assert_eq!(solve_puzzle(line, 14), *expected);
    }
}

fn main() {
    // gets a line from stdin
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", solve_puzzle(&line, 14));
}
