// https://cses.fi/problemset/task/1083

use std::io::{stdin, Result};

fn run(lines: &[String]) -> String {
    let count = lines[0].parse().unwrap();
    let mut numbers = Vec::with_capacity(count);
    for token in lines[1].split(' ') {
        numbers.push(token.parse().unwrap());
    }
    numbers.sort();
    let mut result = None;
    for n in 1..count {
        if n != numbers[n - 1] {
            result = Some(n);
            break;
        }
    }
    result.unwrap_or(count).to_string()
}

pub fn main() {
    let lines = stdin().lines().collect::<Result<Vec<String>>>().unwrap();
    println!("{}", run(&lines));
}
