use std::{fs, path::absolute};
use itertools::Itertools;
use std::cmp::max;

fn calculate_area(coord1: &(i64, i64), coord2: &(i64, i64)) -> i64{
    (coord1.0 - coord2.0 + 1).abs() * (coord1.1 - coord2.1 + 1).abs()
}

fn part1(red_tile_coords: &Vec<(i64, i64)>) -> i64 {
    // Naive Solution - Just see all the possible areas and pick the greatest
    let mut greatest_rectangle_area = 0;
    for i in 0..red_tile_coords.len() {
        for j in i..red_tile_coords.len() {
            greatest_rectangle_area = max(greatest_rectangle_area, calculate_area(&red_tile_coords[i], &red_tile_coords[j]));
        }
    }
    
    greatest_rectangle_area
}


fn main() {
    let file_contents = fs::read_to_string("day_9/input.txt").expect("Failed to read input");
    let mut red_tiles: Vec<(i64, i64)> = Vec::new();

    for line in file_contents.lines() {
        red_tiles.push(
            line
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        );
    }

    println!("Coords {red_tiles:?}");
    
    let best_area = part1(&red_tiles);
    println!("{best_area}");
}
