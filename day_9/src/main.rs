use std::{collections::HashSet, fs};
use itertools::Itertools;
use std::cmp::max;
use std::iter::FromIterator;

fn is_valid_rect(red_tile_coords: &Vec<(i64, i64)>, coord1: &(i64, i64), coord2: &(i64, i64)) -> bool {
    // If our hashes are in the same line, this is a valid rect
    if coord1.0 == coord2.0 || coord1.1 == coord2.1 {
        return true;
    }

    let mut is_above = false;
    let mut is_below: bool = false;

    // Naive solution - just go through the whole list to check
    for &(x, y) in red_tile_coords {
        if (x, y) == *coord1 || (x, y) == *coord2 {
            continue;
        }

        if x <= coord1.0 && x <= coord2.0 && y <= std::cmp::min(coord1.1, coord2.1) {
            is_above = true;
        } else if y >= coord1.1 && y >= coord2.1 && x >= std::cmp::max(coord1.0, coord2.0) {
            is_below = true;
        }

        if is_above && is_below {
            return true;
        }
    }

    false
}

fn calculate_area(coord1: &(i64, i64), coord2: &(i64, i64)) -> i64{
    (coord1.0 - coord2.0 + 1).abs() * (coord1.1 - coord2.1 + 1).abs()
}

fn visualize_tiles(tile_set: &HashSet<(i64, i64)>) {
    println!("{tile_set:?}");
    // find the max tile number
    let mut max_tile_x: i64 = 0;
    let mut max_tile_y: i64 = 0;

    for (i, j) in tile_set.iter() {
        max_tile_x = max(max_tile_x, *i);
        max_tile_y = max(max_tile_y, *j);
    }

    let width = (max_tile_x + 2) as usize;
    let height = (max_tile_y + 2) as usize;
    let mut tiles: Vec<Vec<String>> = vec![vec![String::from("."); width]; height];

    for &(i, j) in tile_set {
        // tiles[j as usize][i as usize] = format!("# {},{} ", j, i);
        tiles[j as usize][i as usize] = format!("#");
    }

    for row in &tiles { 
        let line = row.join("");
        println!("{}", line);
    }
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

fn part2(red_tile_coords: &Vec<(i64, i64)>) -> i64 {
    // Let's first add all the new hashes
    let mut tile_set: HashSet<(i64, i64)> = HashSet::from_iter(red_tile_coords.iter().cloned());
        
    for i in 0..red_tile_coords.len() {
        let curr = red_tile_coords[i];
        let mut next = (0, 0);
        let mut before = (0, 0);

        if i == 0 {
            before = red_tile_coords[red_tile_coords.len() - 1];
        } else {
            before = red_tile_coords[i-1];
        }

        if i == red_tile_coords.len() - 1 {
            next = red_tile_coords[0];
        } else {
            next = red_tile_coords[i+1];
        }

        println!("Curr: {curr:?}, Next: {next:?}, Before: {before:?}");
        for x in 0..=(curr.0-next.0).abs() {
            for y in 0..=(curr.1-next.1).abs() {
                let mut new_x = x;
                let mut new_y = y;
                if next.0 < curr.0 {
                    new_x = -x;
                }

                if next.1 < curr.1 {
                    new_y = -y;
                }

                // println!("inserting next {:?}", (curr.0 + new_x, curr.1 + new_y));
                tile_set.insert((curr.0 + new_x, curr.1 + new_y));
            }
        }
        
        for x in 0..=(before.0-curr.0).abs() {
            for y in 0..=(before.1-curr.1).abs() {
                let mut new_x = x;
                let mut new_y = y;
                if before.1 < curr.1 {
                    new_y = -y;
                }

                if before.0 < curr.0 {
                    new_x = -x;
                }

                // println!("inserting before {:?}", (curr.0 + new_x, curr.1 + new_y));
                tile_set.insert((curr.0 + new_x, curr.1 + new_y));
            }
        }
        visualize_tiles(&tile_set);
        println!("\n");

    };

    visualize_tiles(&tile_set);
    let mut greatest_rectangle_area = 0;
    for i in 0..red_tile_coords.len() {
        for j in i..red_tile_coords.len() {
            if !is_valid_rect(&red_tile_coords, &red_tile_coords[i], &red_tile_coords[j]) {
                continue;
            }

            greatest_rectangle_area = max(greatest_rectangle_area, calculate_area(&red_tile_coords[i], &red_tile_coords[j]));
        }
    }
    
    greatest_rectangle_area
}

fn main() {
    let file_contents = fs::read_to_string("day_9/example.txt").expect("Failed to read input");
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
    
    let tile_set: HashSet<(i64, i64)> = HashSet::from_iter(red_tiles.iter().cloned());
    // visualize_tiles(&tile_set);

    // let best_area = part1(&red_tiles);
    let best_area_with_green = part2(&red_tiles);

    // visualize_tiles(&red_tiles);
    // println!("{best_area}");
    println!("{best_area_with_green}");
}
