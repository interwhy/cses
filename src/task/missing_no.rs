// https://cses.fi/problemset/task/1083

use std::io::stdin;

fn run() -> Result<String, Box<dyn std::error::Error>> {
    let lines: Vec<String> = stdin().lines().map(|x| x.unwrap()).collect();
    let count = lines[0].parse::<usize>()?;
    let tokens = lines[1].split_ascii_whitespace();
    let mut numbers = Vec::with_capacity(count);
    for token in tokens {
        numbers.push(token.parse::<usize>()?);
    }
    numbers.sort();
    let mut result: Option<usize> = None;
    for n in 1..count {
        if n != numbers[n - 1] {
            result = Some(n);
            break;
        }
    }
    Ok(result.unwrap_or(count).to_string())
}

pub fn main() {
    println!("{}", run().unwrap());
}
