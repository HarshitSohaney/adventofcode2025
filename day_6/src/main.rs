use std::fs;

fn parse_input(file_contents: &str) -> Vec<Vec<&str>> {
    let mut hw = Vec::new();

    for line in file_contents.lines() {
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        hw.push(line_vec);
    }

    hw
}

// fn clean_input(hw: &Vec<Vec<&str>>) -> Vec<Vec<&str>> {
//     let mut new_hw: Vec<Vec<&str>> = Vec::new();

//     for row in 0..hw[0].len() {
//         let vec: Vec<&str> = Vec::new();

//         for col in 0..hw.len()-1 {
            
//         }
//     }
// }

fn main() {
    let file_contents = fs::read_to_string("day_6/input.txt").unwrap();
    let hw = parse_input(&file_contents);

    println!("{:?}", hw);

    let operators = &hw[hw.len() - 1];
    let numbers_to_count = hw.len();
    let mut count = 0;

    for idx in 0..hw[0].len() {
        let mut operation: i64 = 0;
        match operators[idx] {
            "*" => {
                operation = 1;
                for i in 0..hw.len()-1 {
                    println!("{:?}", hw[i][idx]);
                    operation = operation * hw[i][idx].parse::<i64>().unwrap();
                }
            },
            "+" => {
                for i in 0..hw.len()-1 {
                    println!("{:?}", hw[i][idx]);
                    operation = operation + hw[i][idx].parse::<i64>().unwrap();
                }
            },
            &_ => {
                // do nothing
            }
        }

        count += operation;
    }

    println!("{count}");
}
