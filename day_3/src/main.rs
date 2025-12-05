use std::fs;
use std::collections::HashSet;

trait FindBestJoltage {
    fn find_best_joltage(&self) -> i64;
    fn biggest_number(&self, sorted_vec: &[Battery], length_of_num: usize, ignore_pos: &mut HashSet<usize>, min_pos: usize) -> i64;
}

#[derive(Debug, Clone)]
struct Battery {
    joltage: i64,
    position: usize,
}

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<Battery>,
}

impl FindBestJoltage for Bank {
    fn find_best_joltage(&self) -> i64 {
        let mut sorted_vec = self.batteries.clone();

        sorted_vec.sort_by_key(|b| -b.joltage);
        let mut ignore_pos = HashSet::new();

        return self.biggest_number(&sorted_vec, 12, &mut ignore_pos, 0);
    }

    fn biggest_number(&self, 
        batteries: &[Battery], 
        length_of_num: usize, 
        ignore_pos: &mut HashSet<usize>,  
        min_pos: usize
    ) -> i64 {
        if length_of_num == 0 {
            return 0;
        }


        for (idx, battery) in batteries.iter().enumerate() {
            if ignore_pos.contains(&idx) 
                || battery.position + length_of_num > batteries.len()
                || battery.position < min_pos {
                continue;
            }

            ignore_pos.insert(idx);
            let tail = self.biggest_number(batteries, length_of_num - 1, ignore_pos, battery.position);

            return battery.joltage * 10_i64.pow((length_of_num - 1) as u32) + tail;
        }

        0
    }
}

fn read_lines(filename: &str) -> Vec<Bank> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        let digits: Vec<i64> = line.chars()
                        .map(|c| c.to_digit(10).unwrap() as i64)
                        .collect();
        
        let batteries: Vec<Battery> = digits
                        .into_iter()
                        .enumerate()
                        .map(|(i, j)| Battery {
                            joltage: j,
                            position: i,
                        })
                        .collect();
        
        result.push(Bank { batteries })
    }
    
    result
}

fn main() {
    let input = read_lines("input.txt");
    let mut count: i64 = 0;

    for bank in input.iter() { 
        let best_joltage = bank.find_best_joltage();
        count += best_joltage;
    }

    println!("{}", count);
}
