use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut ranges: Vec<Vec<i64>> = Vec::new();
    let mut idx: usize = 0;
    for line in file_contents.lines() {
        idx+=1;

        if line == "" {
            break;
        }

        let tuple: Vec<i64> = line.split('-').map(|c| c.parse::<i64>().unwrap()).collect();
        // println!("{:?}",tuple);

        ranges.push(tuple);
    }

    let mut nums: Vec<i64> = Vec::new();
    for line in file_contents.lines().skip(idx) {
        let num: i64 = line.parse::<i64>().unwrap();
        // println!("{:?}", num);
        nums.push(num);
    }

    ranges.sort();

    let mut count: i32 = 0;
    for num in nums {
        for minmax in &ranges {
            if num < minmax[0] {
                break;
            } else if num > minmax[1] {
                // println!("{num} is spoiled in this");
                continue;
            }
            
            count += 1;
            break;
        }
    }

    println!("{count}");
}
