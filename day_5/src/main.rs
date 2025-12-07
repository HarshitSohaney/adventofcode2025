use std::fs;
use itertools::Itertools;

fn merge_ranges(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut count: i64 = 0;
    /** 
     * 3 --- 5
     *     4 --------  14
     *       5 --- 10 
     *               13 --------- 20
     */

    let (mut lowest_min, mut highest_max) = ranges[0];

    for idx in 1..ranges.len(){
        let (new_min, new_max) = ranges[idx];

        if new_min > highest_max {
            count += highest_max - lowest_min + 1;

            lowest_min = new_min;
            highest_max = new_max;
        }

        if new_min < lowest_min {
            lowest_min = new_min;
        }

        if new_max > highest_max {
            highest_max = new_max;
        }
    }

    count += highest_max - lowest_min + 1;

    count
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut idx: usize = 0;
    for line in file_contents.lines() {
        idx+=1;

        if line == "" {
            break;
        }

        let tuple: (i64, i64) = line.split('-').map(|c| c.parse::<i64>().unwrap()).collect_tuple().unwrap();

        ranges.push(tuple);
    }

    let mut nums: Vec<i64> = Vec::new();
    for line in file_contents.lines().skip(idx) {
        let num: i64 = line.parse::<i64>().unwrap();
        nums.push(num);
    }

    ranges.sort();

    println!("{:?}", merge_ranges(&ranges));
}
