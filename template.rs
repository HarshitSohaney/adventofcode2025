use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> i64 {
    // TODO: implement
    0
}

fn part2(input: &str) -> i64 {
    // TODO: implement
    0
}