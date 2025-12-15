use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
struct Device {
    name: String,
    connected_to: HashSet<String>,
    relaxed: bool
}

fn path_finder(
    dag: &mut HashMap<String, Device>, 
    curr_device: String, 
    cache: &mut HashMap<(String, bool, bool), i64>,
    visited_dac: bool,
    visited_fft: bool
) -> i64 {
    // println!("-> {curr_device} v_dac = {visited_dac} , v_fft = {visited_fft}    ");
    if curr_device == "out" {
        if visited_dac && visited_fft { 
            // println!("visited both!! returning 1");
            return 1;
        }

        return 0;
    }

    let key = (curr_device, visited_dac, visited_fft);
     if let Some(v) = cache.get(&key) {
        // println!("Found {} in cache with val {v}", key.0);
        return *v;
    }

    let neighbors: Vec<String> = dag
        .get(&key.0)
        .expect("device not found")
        .connected_to
        .iter()
        .cloned()
        .collect();

    // println!("neigh-> {:?}", neighbors);

    let mut c = 0;
    for device in neighbors {
        if dag.get(&device).map(|d| d.relaxed).unwrap_or(false) {
            continue;
        }

        // for part 1, just remove the checks for dac and fft
        if device == "dac" {
            c += path_finder(dag, device, cache, true, visited_fft);
        } else if device == "fft" {
            c += path_finder(dag, device, cache, visited_dac, true);
        } else {
            c += path_finder(dag, device, cache, visited_dac, visited_fft);
        }
    }

    // println!("We can take {c} paths out of {} to out while crossing dac and fft\n", key.0);
    cache.insert((key.0, visited_dac, visited_fft), c);
    c
}

fn part1(dag: &mut HashMap<String, Device>) {
    let mut cache: HashMap<(String, bool, bool), i64> = HashMap::new();
    let res = path_finder(dag, String::from("you"), &mut cache, false, false);
    println!("{}", res);
}

fn part2(dag: &mut HashMap<String, Device>) {
    let mut cache: HashMap<(String, bool, bool), i64> = HashMap::new();

    let res = path_finder(dag, String::from("svr"), &mut cache, false, false);
    println!("{}", res);
}

fn main() {
    let file_contents = fs::read_to_string("day_11/input.txt").expect("Failed to read input");
    let mut dag: HashMap<String, Device> = HashMap::new();

    for line in file_contents.lines() {
        if let Some((key, rest)) = line.split_once(':') {
            let key = key.trim().to_string();
            let devices: HashSet<String> = rest
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            dag.insert(
                key.clone(),
                Device {
                    name: key,
                    connected_to: devices,
                    relaxed: false
                },
            );
        }
    }

    // Start at you
    // part1(&mut dag);
    part2(&mut dag);
    // println!("{dag:?}");
}
