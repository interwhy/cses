// https://cses.fi/problemset/task/1068

use std::io::stdin;

fn run(line: &str) -> String {
    let mut n = line.trim().parse::<usize>().unwrap();
    let mut result = Vec::with_capacity(10_000);
    result.push(n.to_string());
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        result.push(n.to_string());
    }
    result.join(" ")
}

pub fn main() {
    let line = stdin().lines().next().unwrap().unwrap();
    println!("{}", run(&line));
}
