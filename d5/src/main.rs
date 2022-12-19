use std::io::BufRead;

fn parse_crate_line(line: &str, num: usize) -> Vec<Option<char>> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut result: Vec<Option<char>> = Vec::new();

    for i in (1..num*4).step_by(4) {
        if i > chars.len() {
            result.push(None);
        } else {
            let c = chars[i];
            if c == ' ' {
                result.push(None);
            } else {
                result.push(Some(c));
            }
        }
    }

    result
}


#[derive(Debug, PartialEq, Eq)]
struct Command {
    from: usize,
    to: usize,
    count: usize,
}


/*
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
 */
fn parse_command_line(line: &str) -> Command {
    let mut parts = line.split_whitespace();
    parts.next();
    let count = parts.next().unwrap().parse::<usize>().unwrap();
    parts.next();
    let from = parts.next().unwrap().parse::<usize>().unwrap();
    parts.next();
    let to = parts.next().unwrap().parse::<usize>().unwrap();
    Command { from, to, count }
}

#[test]
fn parse_command_line_test() {
    let command = parse_command_line("move 1 from 2 to 1");
    assert_eq!(command, Command { from: 2, to: 1, count: 1 });
    let command = parse_command_line("move 17 from 5 to 9");
    assert_eq!(command, Command { from: 5, to: 9, count: 17 });
}


fn get_two_elements_mut<'a, T>(v: &'a mut Vec<T>, i: usize, j: usize) -> (&'a mut T, &'a mut T) {
    if i > j {
        let (a, b) = get_two_elements_mut(v, j, i);
        return (b, a);
    }

    let (first, rest) = v.split_at_mut(i+1);
    (&mut first[i], &mut rest[j-i-1])
}

#[test]
fn test_get_two_elements_mut() {
    let mut v = vec![1, 2, 3, 4, 5];
    let (a, b) = get_two_elements_mut(&mut v, 1, 3);
    *a = 10;
    *b = 20;
    assert_eq!(v, vec![1, 10, 3, 20, 5]);

    let (a, b) = get_two_elements_mut(&mut v, 3, 1);
    *a = 10;
    *b = 20;
    assert_eq!(v, vec![1, 20, 3, 10, 5]);
}

fn main() {
    let lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    // find first line that is empty
    let split_point = lines.iter().position(|line| line.is_empty()).unwrap();
    let (crates_lines, command_lines) = lines.split_at(split_point+1);

    let mut crates_lines: Vec<_> = crates_lines.iter().collect();
    crates_lines.pop();
    crates_lines.pop();
    let num_crates = lines[split_point-1].chars().count()/4 + 1;

    let mut crates: Vec<Vec<char>> = Vec::new();
    crates.resize(num_crates, Vec::new());

    for line in crates_lines {
        let parsed = parse_crate_line(&line, num_crates);
        for (j, c) in parsed.iter().enumerate() {
            if let Some(c) = c {
                crates[j].push(*c);
            }
        }
    }

    for stack in crates.iter_mut() {
        stack.reverse();
    }

    for command_line in command_lines {
        let command = parse_command_line(command_line);
        let (from, to) = get_two_elements_mut(&mut crates, command.from-1, command.to-1);

        let mut moved = from.split_off(from.len() - command.count);
        to.append(&mut moved);
    }

    let ans: String = crates.iter()
        .map(|stack| stack.last().unwrap())
        .collect();

    println!("{}", ans);

}
