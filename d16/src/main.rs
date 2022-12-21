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

    let time = 30;

    // time, node, mask
    let mut dp = vec![vec![vec![-5000000; max_mask + 1]; node_count]; time];

    dp[0][start_node][0] = 0;

    for ct in 0..(time - 1) {
        for cnode in 0..node_count {
            for mask in 0..max_mask {
                if dp[ct][cnode][mask] < 0 {
                    continue;
                }

                if vertices[cnode].flow > 0 {
                    let new_mask = mask | (1 << node_to_mask_position[cnode]);
                    if new_mask != mask {
                        let new_time = ct + 1;
                        let time_left = (time - new_time) as i32;
                        let new_flow = dp[ct][cnode][mask] + vertices[cnode].flow * time_left;
                        if new_flow > dp[new_time][cnode][new_mask] {
                            dp[new_time][cnode][new_mask] = new_flow;
                        }
                    }
                }

                for next_node in &vertices[cnode].edges_to {
                    let new_time = ct + 1;
                    let new_mask = mask;
                    if dp[ct][cnode][mask] > dp[new_time][*next_node][new_mask] {
                        dp[new_time][*next_node][new_mask] = dp[ct][cnode][mask];
                    }
                }

                // just stay here
                let new_time = ct + 1;
                let new_mask = mask;
                if dp[ct][cnode][mask] > dp[new_time][cnode][new_mask] {
                    dp[new_time][cnode][new_mask] = dp[ct][cnode][mask];
                }
            }
        }
    }

    let answer = dp[time - 1].iter().flat_map(|v| v.iter()).max().unwrap();

    println!("{}", answer);
}
