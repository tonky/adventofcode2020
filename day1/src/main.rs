use itertools::Itertools;
use std::fs;

fn main() -> std::io::Result<()> {
    let nums = fs::read_to_string("input1.txt")?
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .combinations(3)
        .filter(|c| c.iter().sum::<i32>() == 2020)
        .flatten()
        .collect::<Vec<i32>>();

    println!(">> a * b * c = {}", nums.iter().product::<i32>());

    Ok(())
}
