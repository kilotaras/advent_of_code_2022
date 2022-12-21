use petgraph::prelude::*;
use std::collections::HashMap;
use std::io::{self, Read};

use petgraph::dot::{Config, Dot};

#[derive(Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
struct NodeType {
    name: [char; 2],
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.name[0], self.name[1])
    }
}

impl std::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}{}'", self.name[0], self.name[1])
    }
}

// implements parse for NodeType
impl std::str::FromStr for NodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let name = [chars.next().unwrap(), chars.next().unwrap()];
        Ok(NodeType { name })
    }
}

#[derive(Debug)]
struct VertexInfo {
    name: NodeType,
    flow: i32,
    edges_to: Vec<NodeType>,
}

fn parse_info_line(s: &str) -> VertexInfo {
    let mut lines = s.split(|c| " =;,".contains(c));
    let name = lines.nth(1).unwrap().parse().unwrap();
    let flow = lines.nth(3).unwrap().parse().unwrap();
    let edges_to = lines
        .skip(5)
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().unwrap())
        .collect();
    VertexInfo {
        name,
        flow,
        edges_to,
    }
}

fn parse_info_lines(input: &str) -> Vec<VertexInfo> {
    input.lines().map(parse_info_line).collect()
}

// Floyd-Warshall implementation is bugged on undirected graphs, so we're using directed graphs
// See: https://github.com/petgraph/petgraph/pull/487
type Graph = DiGraphMap<NodeType, ()>;

fn make_graph(info: &[VertexInfo]) -> Graph {
    let edges = info
        .iter()
        .flat_map(|v| v.edges_to.iter().map(|e| (v.name.clone(), e.clone())));

    Graph::from_edges(edges)
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

    let flows: HashMap<NodeType, i32> = info.iter().map(|v| (v.name, v.flow)).collect();

    let graph = make_graph(&info);

    let start_node = NodeType { name: ['A', 'A'] };
    let important_nodes: Vec<_> = flows
        .iter()
        .filter(|&(&name, &flow)| flow != 0 || name == start_node)
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();

    let get_position = |node: &NodeType| {
        important_nodes.iter().position(|n| *n == *node)
    };
    let start_node = get_position(&start_node).unwrap();
    // runs a Floyd-Warshall algorithm to find the shortest path between all pairs of nodes
    let shortest_paths = petgraph::algo::floyd_warshall(&graph, |_| 1).unwrap();

    let shortest_paths = {
        let mut map = HashMap::new();
        for ((left, right), dist) in shortest_paths.iter() {
            let left = get_position(left);
            let right = get_position(right);
            if left.is_some() && right.is_some() {
                let left = left.unwrap();
                let right = right.unwrap();
                map.insert((left, right), *dist);
            }
        }
        map
    };

    let time = 30;
    let node_count = important_nodes.len();
    let max_mask = (1 << node_count) - 1;

    // time, node, mask
    let mut dp = vec![vec![vec![-5000000; max_mask + 1]; node_count]; time];

    dp[0][start_node][0] = 0;

    for ct in 0..time {
        for cnode in 0..node_count {
            for mask in 0..max_mask {
                if dp[ct][cnode][mask] < 0 {
                    continue;
                }

                if mask & (1 << cnode) == 0 {
                    // we can turn on the node
                    let new_mask = mask | (1 << cnode);
                    let new_time = ct + 1;
                    let time_left = (time - new_time) as i32;
                    let new_flow = dp[ct][cnode][mask] + flows[&important_nodes[cnode]] * time_left;
                    if new_time < time && new_flow > dp[new_time][cnode][new_mask] {
                        dp[new_time][cnode][new_mask] = new_flow;
                    }
                }

                for next_node in 0..node_count {
                    // move to the node
                    let d = shortest_paths[&(cnode, next_node)] as usize;
                    let new_time = ct + d;
                    let new_mask = mask;
                    if new_time < time && dp[ct][cnode][mask] > dp[new_time][next_node][new_mask] {
                        dp[new_time][next_node][new_mask] = dp[ct][cnode][mask];
                    }
                }
            }
        }
    }

    let answer = dp[time - 1].iter().flat_map(|v| v.iter()).max().unwrap();

    println!("{}", answer);
}
