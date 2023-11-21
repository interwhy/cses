// https://cses.fi/problemset/task/1069

use std::io::stdin;

fn run() -> Result<String, Box<dyn std::error::Error>> {
    let line: String = stdin().lines().next().unwrap()?;
    let mut max_count = 0;
    let mut count = 0;
    let mut previous: Option<char> = None;
    for ch in line.chars() {
        if Some(ch) != previous {
            max_count = max_count.max(count);
            count = 0;
        }
        previous = Some(ch);
        count += 1;
    }
    max_count = max_count.max(count);
    Ok(max_count.to_string())
}

pub fn main() {
    println!("{}", run().unwrap());
}
