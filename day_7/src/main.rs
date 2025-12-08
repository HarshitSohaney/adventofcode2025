use std::fs;
use std::fmt;
use colorized::*;

#[derive(Debug)]
struct Node {
    value: char,
    count: i64,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Node>>,
    // count: i32,
}

impl Graph {
    fn num_of_splits(&mut self, curr_row: usize, curr_col: usize) -> i64 {
        if curr_row >= self.nodes.len() {
            // we've reached the end, lets return with no splits
            return 1;
        }

        if self.nodes[curr_row][curr_col].count > 0 {
            return self.nodes[curr_row][curr_col].count;
        }

        if curr_col >= self.nodes[0].len() {
            // out of bounds
            return 0;
        }

        match self.nodes[curr_row][curr_col].value {
            '.' => {
                // -- Part 1 --
                // self.nodes[curr_row][curr_col].value = '|';
                // self.num_of_splits(curr_row + 1, curr_col);
                
                // -- Part 2 -- 
                self.nodes[curr_row][curr_col].count += self.num_of_splits(curr_row + 1, curr_col);
            },
            '|' => {
                // do nothing, go no further
            }, 
            '^' => {
                // ---- Part 1 ----
                // if (self.nodes[curr_row][curr_col-1].value != '|'){
                //     self.count += 1;
                // }

                self.nodes[curr_row][curr_col].count += self.num_of_splits(curr_row, curr_col + 1);
                
                // ---- Part 1 ----
                // if (self.nodes[curr_row][curr_col+1].value != '|'){
                //     self.count += 1;
                // }

                self.nodes[curr_row][curr_col].count += self.num_of_splits(curr_row, curr_col - 1);
            },
            _ => {
                // do nothing
            }
        }
        self.nodes[curr_row][curr_col].count
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.nodes {
            for node in row {
                if node.value == '|' {
                    write!(f, "{}({})  ", node.value.to_string().color(Colors::BrightGreenFg), node.count)?;
                } else {
                    write!(f, "{}({})  ", node.value, node.count)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn make_graph_from_str(data: &str) -> Graph {
    let mut nodes = Vec::new();

    for (i, line) in data.lines().enumerate() {
        let line_vec: Vec<Node> = line
                                    .chars()
                                    .enumerate()
                                    .map(|(j, c)| {
                                        Node { value: c, count: 0 }
                                    })
                                    .collect();
        nodes.push(line_vec);
    }

    Graph { nodes }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut graph: Graph = make_graph_from_str(&file_contents);

    graph.num_of_splits(1, graph.nodes[0].len()/2);
    println!("{}", graph.nodes[2][graph.nodes[0].len()/2].count);
}
