use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct VertexInfoStr {
    name: String,
    flow: i32,
    edges_to: Vec<String>,
}

fn parse_info_line(s: &str) -> VertexInfoStr {
    let mut lines = s.split(|c| " =;,".contains(c));
    let name = lines.nth(1).unwrap().to_string();
    let flow = lines.nth(3).unwrap().parse().unwrap();
    let edges_to = lines
        .skip(5)
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect();
    VertexInfoStr {
        name,
        flow,
        edges_to,
    }
}

fn parse_info_lines(input: &str) -> Vec<VertexInfoStr> {
    input.lines().map(parse_info_line).collect()
}

struct VertexInfoIdx {
    name: usize,
    flow: i32,
    edges_to: Vec<usize>,
}

enum Action {
    Stay,
    Move(usize),
    TurnOn(usize, i32),
}

type State = (usize, usize, usize, usize);

fn apply_action(s: State, my: &Action, elephant: &Action) -> (State, i32) {
    let (ct, mut my_node, mut elephant_node, mut mask) = s;

    match (my, elephant) {
        (Action::TurnOn(p1, _), Action::TurnOn(p2, _)) if p1 == p2 => return (s, -1000000),
        _ => (),
    }

    fn turn_on(mask: usize, position: usize) -> usize {
        assert!(mask & (1 << position) == 0);
        mask | (1 << position)
    }

    let mut value: i32 = 0;

    match my {
        Action::Stay => (),
        Action::Move(next_node) => my_node = *next_node,
        Action::TurnOn(position, b) => {
            value += b;
            mask = turn_on(mask, *position);
        },
    }

    match elephant {
        Action::Stay => (),
        Action::Move(next_node) => elephant_node = *next_node,
        Action::TurnOn(position, b) => {
            value += b;
            mask = turn_on(mask, *position);
        },
    }

    let my_new_node = std::cmp::min(my_node, elephant_node);
    let elephant_new_node = std::cmp::max(my_node, elephant_node);
    ((ct + 1, my_new_node, elephant_new_node, mask), value)
}

fn main() {
    let input = {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_to_string(&mut buffer).unwrap();
        buffer
    };

    let info = parse_info_lines(&input);

    let name_to_index: HashMap<String, usize> = info
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.clone(), i))
        .collect();

    let vertices = {
        let mut vertices = info
            .iter()
            .map(|v| VertexInfoIdx {
                name: name_to_index[&v.name],
                flow: v.flow,
                edges_to: v.edges_to.iter().map(|s| name_to_index[s]).collect(),
            })
            .collect::<Vec<_>>();
        vertices.sort_by_key(|v| v.name);
        vertices
    };

    for (i, v) in vertices.iter().enumerate() {
        assert_eq!(i, v.name);
    }

    let interesting_nodes = vertices.iter().filter(|v| v.flow > 0).collect::<Vec<_>>();

    let node_to_mask_position = {
        let mut result = vec![100; vertices.len()];
        for (i, node) in interesting_nodes.iter().enumerate() {
            result[node.name] = i;
        }
        result
    };

    let mask_len = interesting_nodes.len();
    let max_mask = 1 << mask_len;

    let start_node = name_to_index["AA"];
    let node_count = vertices.len();

    let time = 26;

    // time, my_node, elephant_node, mask
    let mut dp = vec![vec![vec![vec![-5000000; max_mask + 1]; node_count]; node_count]; time];

    dp[0][start_node][start_node][0] = 0;

    for ct in 0..(time - 1) {
        println!("ct = {}", ct);
        for my_node in 0..node_count {
            for elephant_node in 0..node_count {
                for mask in 0..max_mask {
                    if dp[ct][my_node][elephant_node][mask] < 0 {
                        continue;
                    }

                    let actions_from_position = |position: usize| {
                        let mut actions = vec![];
                        for &next_node in vertices[position].edges_to.iter() {
                            actions.push(Action::Move(next_node));
                        }

                        let mask_position = node_to_mask_position[position];
                        if mask_position < 100 {
                            if mask & (1 << mask_position) == 0 {
                                let time_left = (time - ct - 1) as i32;
                                let benefit = vertices[position].flow * time_left;
                                actions.push(Action::TurnOn(mask_position, benefit));
                            }
                        }
                        actions
                    };

                    let my_actions = actions_from_position(my_node);
                    let elephant_actions = actions_from_position(elephant_node);

                    let state = (ct, my_node, elephant_node, mask);
                    let cur_value = dp[ct][my_node][elephant_node][mask];
                    for my_action in my_actions.iter() {
                        for elephant_action in elephant_actions.iter() {
                            let (state, benefit) = apply_action(state, my_action, elephant_action);
                            let (new_time, new_my_node, new_elephant_node, new_mask) = state;
                            let new_value = &mut dp[new_time][new_my_node][new_elephant_node][new_mask];
                            *new_value = std::cmp::max(*new_value, cur_value + benefit);
                        }
                    }
                }
            }
        }
    }

    let answer = dp[time - 1].iter().flat_map(|v| v.iter()).flat_map(|v| v.iter()).max().unwrap();

    println!("{}", answer);
}
