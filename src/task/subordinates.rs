// https://cses.fi/problemset/task/1674

use std::io::{stdin, Result};

#[derive(Debug)]
struct TreeNode {
    pub children: Vec<usize>,
}

fn run(lines: &[String]) -> String {
    let employee_count = lines[0].parse::<usize>().unwrap();
    let bosses: Vec<usize> = lines[1]
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect();

    let mut tree = Vec::with_capacity(employee_count);
    tree.resize_with(employee_count, || TreeNode {
        children: Vec::new(),
    });

    for (i, boss) in bosses.iter().enumerate() {
        tree[*boss].children.push(i + 1);
    }

    let mut subordinates = Vec::with_capacity(employee_count);
    subordinates.resize(employee_count, None);
    calculate_subordinates(0, &tree, &mut subordinates);

    subordinates
        .iter()
        .map(|s| s.unwrap_or_default().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn calculate_subordinates(
    node_index: usize,
    tree: &Vec<TreeNode>,
    subordinates: &mut Vec<Option<usize>>,
) -> usize {
    if let Some(s) = subordinates[node_index] {
        return s + 1;
    }

    let node = &tree[node_index];
    if node.children.is_empty() {
        subordinates[node_index] = Some(0);
        return 1;
    }

    let mut result = 0;
    for child in &node.children {
        result += calculate_subordinates(*child, tree, subordinates);
    }

    subordinates[node_index] = Some(result);
    result + 1
}

pub fn main() {
    let lines = stdin().lines().collect::<Result<Vec<String>>>().unwrap();
    println!("{}", run(&lines));
}
