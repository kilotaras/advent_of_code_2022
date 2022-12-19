use id_tree::*;
use std::cell::Cell;
use std::collections::HashMap;
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

    let mut commands = input_lines
        .split_inclusive(|line| line.starts_with("$"))
        .map(|command_lines| {
            let mut command_lines = command_lines.to_vec();
            command_lines.reverse();
            parse_command(&command_lines)
        })
        .collect::<Vec<_>>();
    commands.reverse();
    commands
}

#[derive(Debug, Eq, PartialEq)]
enum NodeType {
    Directory,
    File,
}

struct NodeData {
    size: Cell<Option<usize>>,
    node_type: NodeType,
}

fn main() {
    use id_tree::InsertBehavior::*;
    let commands = get_commands();

    let mut tree: Tree<NodeData> = {
        let cell = Cell::new(None);
        let data = NodeData {
            size: cell,
            node_type: NodeType::Directory,
        };
        TreeBuilder::new().with_root(Node::new(data)).build()
    };

    let root = tree.root_node_id().unwrap().clone();
    let mut cur_node_id = root.clone();
    let mut parent_to_child: HashMap<(NodeId, String), NodeId> = HashMap::new();

    for command in commands {
        match command {
            Command::Change { to } => {
                cur_node_id = match to.as_str() {
                    "/" => root.clone(),
                    ".." => tree.get(&cur_node_id).unwrap().parent().unwrap().clone(),
                    _ => parent_to_child.get(&(cur_node_id, to)).unwrap().clone(),
                }
            }
            Command::Ls { entries } => {
                for entry in entries {
                    let size = match entry.answer_type {
                        LsEntryType::Directory => None,
                        LsEntryType::File(size) => Some(size),
                    };
                    let nodeType = match entry.answer_type {
                        LsEntryType::Directory => NodeType::Directory,
                        LsEntryType::File(_) => NodeType::File,
                    };
                    let nodeData = NodeData {
                        size: Cell::new(size),
                        node_type: nodeType,
                    };
                    let node = Node::new(nodeData);
                    let node_id = &mut tree.insert(node, UnderNode(&cur_node_id)).unwrap();
                    parent_to_child.insert((cur_node_id.clone(), entry.name), node_id.clone());
                }
            }
        }
    }

    let nodes = tree.traverse_post_order(&root).unwrap();

    let mut total = 0;

    for node in nodes {
        let data = node.data().size.get();

        if data.is_none() {
            let size = node
                .children()
                .iter()
                .map(|child_id| tree.get(child_id).unwrap().data())
                .map(|data| data.size.get().unwrap())
                .sum::<usize>();

            node.data().size.set(Some(size));
        }
        let size = node.data().size.get().unwrap();
        if size <= 100000 && node.data().node_type == NodeType::Directory {
            total += size;
        }
    }

    println!("{}", total);
}
