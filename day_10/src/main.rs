use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::min;
use regex::Regex;
use good_lp::{
    variables, variable, Expression, ProblemVariables, SolverModel,
    default_solver, Solution, constraint
};

type CacheKey = (Vec<State>, Vec<bool>);

#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
enum State {
    On,
    Off,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Button {
    idxs: Vec<usize>,  // each (...) as a Vec<i32>
}

#[derive(Debug)]
struct Manual {
    lights: Vec<State>,   // from [...]
    buttons: Vec<Button>, // from (..)(..)(..)
    joltages: Vec<i32>, // from {...}
}

fn create_manuals_from_line(line: &str) -> Manual {
    let re_square = Regex::new(r"\[([^\]]+)\]").unwrap();
    let re_parens = Regex::new(r"\(([^)]+)\)").unwrap();
    let re_braces = Regex::new(r"\{([^}]+)\}").unwrap();

    let mut buttons: Vec<Button> = Vec::new();
    let mut lights: Vec<State> = Vec::new();
    let mut joltages: Vec<i32> = Vec::new();
    // Extract [ ... ]
    let square = re_square
            .captures(line)
            .map(|cap| cap[1].to_string())
            .expect("No [...] found in line");

    for light in square.chars() {
        match light {
            '.' => lights.push(State::Off),
            '#' => lights.push(State::On),
            _   => panic!("Unexpected char in light pattern: {light}"),
        }
    }

    // Extract () () buttons
    for cap in re_parens.captures_iter(line) {
        let idxs = cap[1]
            .split(',')
            .map(|s| s.trim().parse::<usize>().expect("Failed to parse index in ()"))
            .collect::<Vec<usize>>();

        buttons.push(Button { idxs });
    }

    if let Some(cap) = re_braces.captures(line) {
        joltages = cap[1]
            .split(',')
            .map(|s| s.trim().parse::<i32>().expect("Failed to parse number in {}"))
            .collect::<Vec<i32>>();
    }

    Manual {
        lights,
        buttons,
        joltages
    }
}

fn min_count_joltage_dfs(expected_jolts: &Vec<i32>, buttons: &Vec<Button>) -> i32 {
    let n = expected_jolts.len();

    // Quick reachability check:
    // If some counter needs >0 but no button ever touches it, it's impossible.
    let mut touched = vec![false; n];
    for button in buttons {
        for &idx in &button.idxs {
            if idx < n {
                touched[idx] = true;
            }
        }
    }
    for (i, &e) in expected_jolts.iter().enumerate() {
        if e > 0 && !touched[i] {
            return i32::MAX;
        }
    }

    // how much is left to reach the target
    let mut remaining = expected_jolts.clone();
    let mut memo: HashMap<(usize, Vec<i32>), i32> = HashMap::new();

    fn dfs(
        idx: usize,
        buttons: &Vec<Button>,
        remaining: &mut [i32],
        memo: &mut HashMap<(usize, Vec<i32>), i32>,
    ) -> i32 {
        // If all counters are satisfied exactly
        if remaining.iter().all(|&x| x == 0) {
            return 0;
        }

        // dead end
        if idx == buttons.len() {
            return i32::MAX;
        }

        // Memoization key: (which button we're considering, current remaining)
        let key = (idx, remaining.to_vec());
        if let Some(&val) = memo.get(&key) {
            return val;
        }

        let button = &buttons[idx];

        // Find the maximum number of times we can press this button
        // without making any remaining[i] go negative.
        let mut max_times = i32::MAX;
        for &i in &button.idxs {
            let rem = remaining[i];
            if rem < 0 {
                // Already impossible state
                memo.insert(key, i32::MAX);
                return i32::MAX;
            }
            if rem < max_times {
                max_times = rem;
            }
        }
        if max_times < 0 {
            memo.insert(key, i32::MAX);
            return i32::MAX;
        }

        let mut best = i32::MAX;

        // Try pressing this button t times (from 0 up to max_times)
        for t in 0..=max_times {
            // Apply: subtract t from each affected counter
            for &i in &button.idxs {
                remaining[i] -= t;
            }

            let sub = dfs(idx + 1, buttons, remaining, memo);
            if sub != i32::MAX {
                best = best.min(sub + t);
            }

            // Undo: add t back
            for &i in &button.idxs {
                remaining[i] += t;
            }
        }

        memo.insert(key, best);
        best
    }

    dfs(0, buttons, &mut remaining, &mut memo)
}

fn min_count_ilp(manual: &Manual) -> Option<i64> {
    let n = manual.joltages.len();
    let m = manual.buttons.len();

    // 1. Define ILP variables x_j >= 0, integer
    let mut vars = variables!();
    let x = vars.add_vector(variable().min(0).integer(), m);
    println!("X is {x:?}");

    let objective: Expression = x.iter().copied().sum();
    // 2. Build the problem: minimize sum_j x_j
    let mut problem = vars.minimise(objective.clone()).using(default_solver);

    // 3. Add constraints: For each joltage counter i:
    //    sum_{j: i in button_j.idxs} x_j == joltages[i]
    for i in 0..n {
        let mut expr = Expression::from(0.0);
        for (j, button) in manual.buttons.iter().enumerate() {
            if button.idxs.contains(&i) {
                expr = expr + x[j];
            } 
        }
        problem = problem.with(constraint::eq(expr, manual.joltages[i]));
    }

    let solution = match problem.solve() {
        Ok(sol) => sol,
        Err(_) => return None,
    };

    // 5. Extract integer result: sum_j x_j
    let total_presses = solution.eval(objective);
    // The model is integer, but good_lp returns f64
    Some(total_presses.round() as i64)
}

fn min_count(curr_lights: &mut Vec<State>, 
        expected_lights: &Vec<State>, 
        buttons: &Vec<Button>,
        available: &mut Vec<bool>,                   // new way to track unused buttons
        cache: &mut HashMap<(Vec<State>, Vec<bool>), i32>) -> i32 {
    if expected_lights == curr_lights {
        return 0;
    }

    if available.is_empty() {
        return i32::MAX;
    }
 
    let key = (curr_lights.clone(), available.clone());
    if let Some(&v) = cache.get(&key) {
        return v;
    }

    let mut c = i32::MAX;
     for (i, button) in buttons.iter().enumerate() {
        if !available[i] {
            continue; // button already used
        }

        // mark used
        available[i] = false;

        for &idx in button.idxs.iter() {
            match curr_lights[idx] {
                State::Off => curr_lights[idx] = State::On,
                State::On => curr_lights[idx] = State::Off,
            }
        }

        let temp = min_count(curr_lights, expected_lights, buttons, available, cache);
        if temp != i32::MAX {
            c = min(c, 1 + temp);
        }

        for &idx in button.idxs.iter() {
            match curr_lights[idx] {
                State::Off => curr_lights[idx] = State::On,
                State::On  => curr_lights[idx] = State::Off,
            }
        }
    }

    cache.insert(key, c);
    c
}

fn part1(manuals: &mut Vec<Manual>) {
    let mut best = 0;

    for manual in manuals {
        let mut curr_lights = vec![State::Off; manual.lights.len()];
        let mut cache = HashMap::new();
        let mut available = vec![true; manual.buttons.len()]; // all buttons initially usable

        let res = min_count(
            &mut curr_lights,
            &manual.lights,
            &manual.buttons,
            &mut available,
            &mut cache,
        );

        best += res;
    }
    
    println!("{best}");
}

fn part2(manuals: &mut Vec<Manual>) {
    let mut total = 0i64;

    for manual in manuals.iter() {
        match min_count_ilp(manual) {
            Some(p) => total += p,
            None => {
                panic!(
                    "Joltage target {:?} unreachable with buttons {:?}",
                    manual.joltages, manual.buttons
                );
            }
        }
    }

    println!("{total}");
}

fn main() {
    let file_contents = fs::read_to_string("day_10/input.txt").expect("Failed to read input");
    let mut manuals: Vec<Manual> = Vec::new();

    for line in file_contents.lines() {
        manuals.push(create_manuals_from_line(line));
    }
    // println!("{manuals:?}\n");
    // part1(&mut manuals);
    part2(&mut manuals);
}
