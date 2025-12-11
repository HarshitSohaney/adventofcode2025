use std::{collections::HashSet, fs};
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::iter::FromIterator;

type Coord = (i64, i64);

// Im so mad I had to gpt how to tell if an edge crosses or not :(

pub fn point_in_polygon(p: Coord, poly: &[Coord]) -> bool {
    let (px, py) = p;
    let mut inside = false;

    // Loop over edges: (v1 -> v2)
    for i in 0..poly.len() {
        let (x1, y1) = poly[i];
        let (x2, y2) = poly[(i + 1) % poly.len()];

        // ---------- 1. Check boundary conditions ----------
        // If the point lies exactly on a horizontal edge:
        if y1 == y2 && py == y1 && px >= x1.min(x2) && px <= x1.max(x2) {
            return true;
        }

        // If the point lies exactly on a vertical edge:
        if x1 == x2 && px == x1 && py >= y1.min(y2) && py <= y1.max(y2) {
            return true;
        }

        // ---------- 2. Ray casting test ----------
        //
        // We cast a ray to the RIGHT of the point.
        // We check if the edge crosses the horizontal line y = py.
        //
        // Conditions for an edge to cross:
        // - The point's y must be between y1 and y2 (strictly).
        // - The intersection x-coordinate must be > px.
        //
        let y_between = (y1 > py) != (y2 > py);  // py is between y1 and y2?

        if y_between {
            // Compute where the edge intersects horizontal line y = py.
            // Since the polygon is axis-aligned, only vertical edges matter here.
            if x1 == x2 { 
                // Vertical edge intersection:
                let x_intersect = x1;
                if x_intersect > px {
                    inside = !inside; // We crossed the edge
                }
            }
            // Horizontal edges are ignored for crossings since they don't change parity.
        }
    }

    inside
}

fn calculate_area(coord1: &Coord, coord2: &Coord) -> i64{
    ((coord1.0 - coord2.0).abs() + 1) * ((coord1.1 - coord2.1).abs() + 1)
}

fn get_bounds(tile_set: &HashSet<Coord>) -> (i64, i64, i64, i64) {
    // (min_x, max_x, min_y, max_y)
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for &(x, y) in tile_set {
        min_x = min(min_x, x);
        max_x = max(max_x, x);
        min_y = min(min_y, y);
        max_y = max(max_y, y);
    }

    (min_x, max_x, min_y, max_y)
}

fn visualize_tiles(tile_set: &HashSet<Coord>, red_tiles: &[Coord]) {
    println!("{tile_set:?}");

    if tile_set.is_empty() {
        println!("<empty tile set>");
        return;
    }

    let (min_x, max_x, min_y, max_y) = get_bounds(tile_set);
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    // Fill with '.'
    let mut tiles: Vec<Vec<char>> = vec![vec!['.'; width]; height];

    let red_set: HashSet<Coord> = HashSet::from_iter(red_tiles.iter().cloned());

    for &(x, y) in tile_set {
        let col = (x - min_x) as usize;
        let row = (y - min_y) as usize;

        tiles[row][col] = if red_set.contains(&(x, y)) { '#' } else { 'X' };
    }

    for row in &tiles {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}

fn part1(red_tile_coords: &Vec<Coord>) -> i64 {
    // Naive Solution - Just see all the possible areas and pick the greatest
    let mut greatest_rectangle_area = 0;
    for i in 0..red_tile_coords.len() {
        for j in i..red_tile_coords.len() {
            greatest_rectangle_area = max(greatest_rectangle_area, calculate_area(&red_tile_coords[i], &red_tile_coords[j]));
        }
    }
    
    greatest_rectangle_area
}

fn rect_corners(a: Coord, c: Coord) -> (Coord, Coord, Coord, Coord) {
    let (x1, y1) = a;
    let (x2, y2) = c;

    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    let a = (min_x, min_y);
    let b = (min_x, max_y);
    let c = (max_x, max_y);
    let d = (max_x, min_y);

    (a, b, c, d)
}

fn rect_bounds(a: Coord, c: Coord) -> (i64, i64, i64, i64) {
    let (x1, y1) = a;
    let (x2, y2) = c;

    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    (min_x, max_x, min_y, max_y)
}

fn part2(red_tile_coords: &Vec<Coord>) -> i64 {
    // Let's first add all the new hashes
    // let mut tile_set = create_boundary(&red_tile_coords);s

    // visualize_tiles(&tile_set, &red_tile_coords);
    let mut greatest_rectangle_area = 0;
    for i in 0..red_tile_coords.len() {
        for j in i+1..red_tile_coords.len() {
            let a = red_tile_coords[i];
            let c = red_tile_coords[j];

            // If the rectangle (possibly thin) is not fully inside/on the polygon, skip it.
            if !rect_inside_polygon(a, c, red_tile_coords) {
                continue;
            }

            // println!("{:?},{:?} is a valid rect", red_tile_coords[i], red_tile_coords[j]);
            greatest_rectangle_area = max(greatest_rectangle_area, calculate_area(&a, &c));
            // println!("Greatest area is now {greatest_rectangle_area}\n\n");
        }
    }
    
    greatest_rectangle_area
}

fn segments_cross_interior(a1: Coord, a2: Coord, b1: Coord, b2: Coord) -> bool {
    let (ax1, ay1) = a1;
    let (ax2, ay2) = a2;
    let (bx1, by1) = b1;
    let (bx2, by2) = b2;

    let a_horizontal = ay1 == ay2;
    let a_vertical   = ax1 == ax2;
    let b_horizontal = by1 == by2;
    let b_vertical   = bx1 == bx2;

    // 1. If both are horizontal or both are vertical (collinear), we treat that as "no bad crossing".
    // That could be the rectangle edge lying along the polygon boundary, which is allowed.
    if (a_horizontal && b_horizontal) || (a_vertical && b_vertical) {
        return false;
    }

    // 2. We only care about mixed horizontal/vertical pairs.
    let (h1, h2, v1, v2) = if a_horizontal && b_vertical {
        (a1, a2, b1, b2)
    } else if a_vertical && b_horizontal {
        (b1, b2, a1, a2)
    } else {
        // Non axis-aligned case shouldn't happen in this puzzle.
        return false;
    };

    let (hx1, hy) = h1;
    let (hx2, _)  = h2;
    let (vx, vy1) = v1;
    let (_, vy2)  = v2;

    let h_min_x = hx1.min(hx2);
    let h_max_x = hx1.max(hx2);
    let v_min_y = vy1.min(vy2);
    let v_max_y = vy2.max(vy1);

    // Check if the infinite lines intersect within both segments.
    if !(vx >= h_min_x && vx <= h_max_x && hy >= v_min_y && hy <= v_max_y) {
        return false;
    }

    let inter = (vx, hy);

    // 3. Ignore intersections exactly at endpoints (corner touches).
    if inter == a1 || inter == a2 || inter == b1 || inter == b2 {
        return false;
    }

    // Anything else is a true crossing.
    true
}

fn edge_inside_polygon(p1: Coord, p2: Coord, poly: &[Coord]) -> bool {
    // Endpoints must be inside or on boundary.
    if !point_in_polygon(p1, poly) || !point_in_polygon(p2, poly) {
        return false;
    }

    // And the segment must not cross the polygon boundary in its interior.
    for i in 0..poly.len() {
        let q1 = poly[i];
        let q2 = poly[(i + 1) % poly.len()];

        if segments_cross_interior(p1, p2, q1, q2) {
            return false;
        }
    }

    true
}

fn rect_inside_polygon(a0: Coord, c0: Coord, poly: &[Coord]) -> bool {
    let (a, b, c, d) = rect_corners(a0, c0);

    edge_inside_polygon(a, b, poly)
        && edge_inside_polygon(b, c, poly)
        && edge_inside_polygon(c, d, poly)
        && edge_inside_polygon(d, a, poly)
}

/// BRUTE FORCE FUNCTIONS
fn is_valid_rect(
    tile_set: &HashSet<Coord>,
    _red_tile_coords: &[Coord],
    coord1: &Coord,
    coord2: &Coord,
) -> bool {
    // If our hashes are in the same line, this is a valid rect
    if coord1.0 == coord2.0 || coord1.1 == coord2.1 {
        return true;
    }

    // Get the two other corners of this rect
    let max_x = max(coord1.0, coord2.0);
    let min_x = min(coord1.0, coord2.0);
    let max_y = max(coord1.1, coord2.1);
    let min_y = min(coord1.1, coord2.1);

    let c3 = (min_x, max_y);
    let c4 = (max_x, min_y);

    // For a valid rectangle, all 4 corners must exist in tile_set.
    tile_set.contains(coord1)
        && tile_set.contains(coord2)
        && tile_set.contains(&c3)
        && tile_set.contains(&c4)
}

fn create_boundary(red_tiles: &[Coord]) -> HashSet<Coord> {
    let mut tile_set: HashSet<Coord> = HashSet::from_iter(red_tiles.iter().cloned());

    for (i, &curr) in red_tiles.iter().enumerate() {
        let next = if i == red_tiles.len() - 1 {
            red_tiles[0]
        } else {
            red_tiles[i + 1]
        };

        let dx = next.0 - curr.0;
        let dy = next.1 - curr.1;

        for x_off in 0..=dx.abs() {
            for y_off in 0..=dy.abs() {
                let mut new_x = x_off;
                let mut new_y = y_off;

                if dx < 0 {
                    new_x = -new_x;
                }
                if dy < 0 {
                    new_y = -new_y;
                }

                tile_set.insert((curr.0 + new_x, curr.1 + new_y));
            }
        }
    }

    tile_set
}

fn fill_island(tile_set: &mut HashSet<Coord>, red_tile_coords: &[Coord]) {
    if tile_set.is_empty() {
        return;
    }

    let (_, max_x, _, max_y) = get_bounds(tile_set);

    let copy_set = tile_set.clone();
    let mut new_set_of_items: HashSet<Coord> = HashSet::new();

    for &(x, y) in copy_set.iter() {
        let mut there_is_a_bottom = false;

        // Check if there's any tile to the right on this row (y)
        for j in x..=max_x {
            if tile_set.contains(&(j, y)) {
                there_is_a_bottom = true;
                break;
            }
        }

        if !there_is_a_bottom {
            continue;
        }

        // Fill from (x,y) horizontally to max_x on this row
        for j in x..=max_x {
            new_set_of_items.insert((j, y));
        }
    }

    tile_set.extend(new_set_of_items);
    visualize_tiles(tile_set, red_tile_coords);
}

fn main() {
    let file_contents = fs::read_to_string("day_9/example.txt").expect("Failed to read input");
    let mut red_tiles: Vec<Coord> = Vec::new();

    for line in file_contents.lines() {
        red_tiles.push(
            line
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap()
        );
    }

    // let best_area = part1(&red_tiles);
    let best_area_with_green = part2(&red_tiles);

    // visualize_tiles(&red_tiles);
    // println!("{best_area}");
    println!("{best_area_with_green}");
}
