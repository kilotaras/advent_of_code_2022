use std::io::BufRead;

#[derive(Debug, Eq, PartialEq)]
enum LsEntryType {
    Directory,
    File(usize),
}

#[derive(Debug, Eq, PartialEq)]
struct LsEntry {
    name: String,
    answer_type: LsEntryType,
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Change { to: String },
    Ls { entries: Vec<LsEntry> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ls_entry() {
        let samples = [
            ("dir e", "e", LsEntryType::Directory),
            ("2557 g", "g", LsEntryType::File(2557)),
            ("62596 h.lst", "h.lst", LsEntryType::File(62596)),
        ];
        for (input, ans_name, ans_type) in samples {
            let entry = parse_ls_entry(input);
            assert_eq!(entry.name, ans_name);
            assert_eq!(entry.answer_type, ans_type);
        }
    }

    #[test]
    fn test_parse_command() {
        let samples = [
            (
                vec!["$ cd /home/user".to_string()],
                Command::Change {
                    to: "/home/user".to_string(),
                },
            ),
            (vec!["$ ls".to_string()], Command::Ls { entries: vec![] }),
            (
                vec![
                    "$ ls".to_string(),
                    "dir e".to_string(),
                    "2557 g".to_string(),
                    "62596 h.lst".to_string(),
                ],
                Command::Ls {
                    entries: vec![
                        LsEntry {
                            name: "e".to_string(),
                            answer_type: LsEntryType::Directory,
                        },
                        LsEntry {
                            name: "g".to_string(),
                            answer_type: LsEntryType::File(2557),
                        },
                        LsEntry {
                            name: "h.lst".to_string(),
                            answer_type: LsEntryType::File(62596),
                        },
                    ],
                },
            ),
        ];
        for (input, ans) in samples {
            assert_eq!(parse_command(&input), ans);
        }
    }
}

fn parse_ls_entry(line: &str) -> LsEntry {
    let mut parts = line.split(" ");
    let first = parts.next().unwrap();
    let name = parts.next().unwrap();

    if first == "dir" {
        LsEntry {
            name: name.to_string(),
            answer_type: LsEntryType::Directory,
        }
    } else {
        let size = first.parse().unwrap();
        LsEntry {
            name: name.to_string(),
            answer_type: LsEntryType::File(size),
        }
    }
}

fn parse_command(lines: &[String]) -> Command {
    let mut lines = lines.iter();
    let command = lines.next().unwrap();
    if command == "$ ls" {
        let entries = lines.map(|line| parse_ls_entry(line)).collect();
        Command::Ls { entries }
    } else {
        let to = command.split(" ").nth(2).unwrap().to_string();
        Command::Change { to }
    }
}

fn get_commands() -> Vec<Command> {
    let mut input_lines: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    // there's no rsplit_inclusive in std, so we reverse the lines and then reverse them back after split
    input_lines.reverse();

    let mut commands = input_lines.split_inclusive(|line| line.starts_with("$")).map(|command_lines| {
        let mut command_lines = command_lines.to_vec();
        command_lines.reverse();
        parse_command(&command_lines)
    }).collect::<Vec<_>>();
    commands.reverse();
    commands
}

fn main() {
    let commands = get_commands();

    dbg!(commands);
}
