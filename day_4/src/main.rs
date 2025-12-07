use std::fs;
use colored::Colorize;

fn make_graph(data: &str) -> Vec<Vec<char>> {
    let mut rolls = Vec::new();

    for line in data.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        rolls.push(line_vec);
    }

    rolls
}

fn is_valid(rolls_graph: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    const DIRS: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 1)
    ];
    let mut count = 0;
    let rows = rolls_graph.len();
    let cols = rolls_graph[0].len();

    for (dx, dy) in DIRS.iter() {
        let x = i as i32 + dx;
        let y = j as i32 + dy;

        if x < 0 || x >= rows as i32 || y < 0 || y >= cols as i32 {
            // Out of bounds!!
            continue;
        }

        let ux = x as usize;
        let uy = y as usize;

        if rolls_graph[ux][uy] == '@' {
            count += 1;
        }

        if count >= 4 {
            return false;
        }
    }

    true
}

fn bfs(rolls_graph: &Vec<Vec<char>>) -> i32 {
    let mut q: VecDeque<(i32, i32)
}

fn find_bad_rolls(rolls_graph: &mut Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    let mut to_mark = Vec::new();

    for (i, line) in rolls_graph.iter().enumerate() {
        for (j, roll) in line.iter().enumerate() {
            if *roll != '@' {
                continue;
            }

            if !is_valid(rolls_graph, i, j) {
                continue;
            }

            to_mark.push((i, j));
        }
    }

    for (i, j) in to_mark {
        rolls_graph[i][j] = '.';
        count += 1;
    }

    count
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut rolls = make_graph(&file_contents);

    for roll_line in rolls.iter() {
        println!("{:?}", roll_line);
    }

    let mut bad_rolls_left = true;
    let mut count = 0;

    while true {
        // keep going until find_bad_rolls is false
        let remove_rolls: i32 = find_bad_rolls(&mut rolls);

        count += remove_rolls;
        if remove_rolls == 0 {
            break;
        }
    }

    for roll_line in rolls.iter() {
        println!("{:?}", roll_line);
    }

    println!("{count}");
}
