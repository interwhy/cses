// https://cses.fi/problemset/task/1624/

use std::io::stdin;

fn run() -> Result<String, Box<dyn std::error::Error>> {
    let lines: Vec<String> = stdin().lines().map(|x| x.unwrap()).collect();
    let mut reserved: u64 = 0;
    let mut i: u64 = 0;
    for line in lines {
        for ch in line.chars() {
            if ch == '*' {
                reserved |= 1 << i;
                i += 1;
            } else if ch == '.' {
                i += 1;
            }
        }
    }
    Ok(queens(reserved, 0).to_string())
}

const DIAGONAL_MASK_BACKWARDS: u64 = 0x0102_0408_1020_4080;
const DIAGONAL_MASK_FORWARDS: u64 = 0x8040_2010_0804_0201;

fn queens(reserved: u64, rank: usize) -> usize {
    if rank >= 8 {
        return 1;
    }
    let rank_mask: u64 = 0xff << (rank * 8);
    let possible: u64 = (!reserved) & rank_mask;
    if possible == 0 {
        return 0;
    }
    let mut total = 0;
    for file in 0..8 {
        let square_mask = 1 << (file + rank * 8);
        if reserved & square_mask == 0 {
            let file_mask: u64 = 0x0101_0101_0101_0101 << file;
            let difference = file as isize - rank as isize;
            let sum = file + rank;
            let diagonal_mask_backwards: u64 = if sum > 7 {
                DIAGONAL_MASK_BACKWARDS << ((sum - 7) * 8)
            } else {
                DIAGONAL_MASK_BACKWARDS >> ((7 - sum) * 8)
            };
            let diagonal_mask_forwards: u64 = if difference > 0 {
                DIAGONAL_MASK_FORWARDS >> (difference * 8)
            } else {
                DIAGONAL_MASK_FORWARDS << (-difference * 8)
            };
            assert_eq!(
                square_mask,
                diagonal_mask_backwards & diagonal_mask_forwards
            );
            total += queens(
                reserved | rank_mask | file_mask | diagonal_mask_backwards | diagonal_mask_forwards,
                rank + 1,
            );
        }
    }
    total
}

pub fn main() {
    println!("{}", run().unwrap());
}
