use std::fs;
use std::error::Error;

trait FindBestJoltage {
    fn find_best_joltage(&self) -> i32;
    fn do_loop(&self, sorted: &[Battery], after_pos: usize, ignore_pos: usize) -> (Battery, usize);
}

#[derive(Debug, Clone)]
struct Battery {
    joltage: i32,
    position: usize,
}

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<Battery>,
}

impl FindBestJoltage for Bank {
    fn find_best_joltage(&self) -> i32 {
        let mut sorted_vec = self.batteries.clone();
        let mut number: i32 = 0;

        sorted_vec.sort_by_key(|b| -b.joltage);
        println!("SORTED VEC {:?}", sorted_vec);
        let (first_battery, first_idx) = self.do_loop(&sorted_vec, usize::MIN, usize::MAX);

        number += first_battery.joltage;
        number *= 10;

        println!("Number so far {number}");
        let (second_battery, second_idx) = self.do_loop(&sorted_vec, first_battery.position + 1, first_idx);

        number + second_battery.joltage
    }

    fn do_loop(&self, sorted: &[Battery], after_pos: usize, ignore_pos: usize) -> (Battery, usize) {
        println!("What are going to: after --> {:?} ignore --> {:?}", after_pos, ignore_pos);
        for (idx, b) in sorted.iter().enumerate() {
            if b.position < after_pos || idx == ignore_pos {
                println!("moving on from {:?}", b);
                continue;
            }

            if b.position == (sorted.len() - 1) && after_pos == 0 {
                continue;
            }
            
            return (b.clone(), idx);
        }

        (Battery { joltage: -1, position: usize::MAX }, usize::MAX)
    }
}

fn read_lines(filename: &str) -> Vec<Bank> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        let digits: Vec<i32> = line.chars()
                        .map(|c| c.to_digit(10).unwrap() as i32)
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

fn part1(input: &[Bank]) -> i32 {
    let mut count: i32 = 0;

    for bank in input.iter() { 
        let bestJoltage = bank.find_best_joltage();
        println!("THIS BANKS BEST JOLTAGE IS {:?}", bestJoltage);
        count += bestJoltage;

        println!("Onto the next! \n");
    }

    count
}

fn part2(input: &[Bank]) -> i32 {
    // TODO: implement
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_lines("input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
