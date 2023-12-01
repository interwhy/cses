// https://cses.fi/problemset/task/1130

use std::io::{stdin, Result};

#[derive(Default, Debug)]
struct Graph {
    nodes: Vec<GraphNode>,
}

#[derive(Default, Debug)]
struct GraphNode {
    adjacent: Vec<usize>,
}

fn parse_graph(lines: &[String]) -> Graph {
    let node_count = lines[0].parse::<usize>().unwrap();
    let mut graph = Graph::default();
    graph.nodes.resize_with(node_count, GraphNode::default);

    for line in lines[1..].iter() {
        let edge: Vec<usize> = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        assert_eq!(edge.len(), 2);
        graph.nodes[edge[0]].adjacent.push(edge[1]);
        graph.nodes[edge[1]].adjacent.push(edge[0]);
    }

    graph
}

fn depth_first_search(
    graph: &Graph,
    visited: &mut [bool],
    edges: &mut usize,
    node: usize,
    parent: usize,
) {
    for &adjacent in graph.nodes[node].adjacent.iter() {
        if adjacent == parent {
            continue;
        }
        depth_first_search(graph, visited, edges, adjacent, node);
        if !visited[adjacent] && !visited[node] {
            *edges += 1;
            visited[adjacent] = true;
            visited[node] = true;
        }
    }
}

fn run(lines: &[String]) -> String {
    let graph = parse_graph(lines);
    let mut visited = Vec::new();
    visited.resize(graph.nodes.len(), false);
    let mut edges = 0;
    depth_first_search(&graph, &mut visited, &mut edges, 0, usize::MAX);
    edges.to_string()
}

pub fn main() {
    let lines = stdin().lines().collect::<Result<Vec<String>>>().unwrap();
    println!("{}", run(&lines));
}
