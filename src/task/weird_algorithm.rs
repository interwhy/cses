// https://cses.fi/problemset/task/1068

use std::io::stdin;

fn run() -> Result<String, Box<dyn std::error::Error>> {
    let line = stdin().lines().next().unwrap()?;
    let mut n = line.trim().parse::<u64>()?;
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
    let s = result.join(" ");
    Ok(s)
}

pub fn main() {
    println!("{}", run().unwrap());
}
