// https://cses.fi/problemset/task/1069

use std::io::stdin;

fn run(line: &str) -> String {
    let mut max_count = 0;
    let mut count = 0;
    let mut previous = None;
    for ch in line.chars() {
        if Some(ch) != previous {
            max_count = max_count.max(count);
            count = 0;
        }
        previous = Some(ch);
        count += 1;
    }
    max_count.max(count).to_string()
}

pub fn main() {
    let line: String = stdin().lines().next().unwrap().unwrap();
    println!("{}", run(line.as_str()));
}
