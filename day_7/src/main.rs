use std::fs;
use std::fmt;
use colorized::*;
use std::time::Instant;

#[derive(Debug)]
struct Node {
    value: char,
    count: Option<i64>, // None = not computed, Some(x) = computed result
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Node>>,
}

impl Graph {
    fn num_of_splits(&mut self, curr_row: usize, curr_col: usize) -> i64 {
        if curr_row >= self.nodes.len() {
            // we've reached the end, lets return with no splits
            return 1;
        }

        if curr_col >= self.nodes[0].len() {
            // out of bounds
            return 0;
        }

        // Already calculated?? Let's return that count!!
        if let Some(cached) = self.nodes[curr_row][curr_col].count {
            /**
             * Note: “If node.count is the Some variant, 
             * take the inner value and bind it to the variable cached.”
             */
            return cached;
        }

        let res = match self.nodes[curr_row][curr_col].value {
            '.' => {
                self.num_of_splits(curr_row + 1, curr_col)
            },
            '^' => {
                let mut total = 0;

                // right
                total += self.num_of_splits(curr_row, curr_col + 1);

                // left
                if curr_col > 0 {
                    total += self.num_of_splits(curr_row, curr_col - 1);
                }

                total
            },
            _ => {
                // do nothing
                0
            }
        };

        self.nodes[curr_row][curr_col].count = Some(res);
        res
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.nodes {
            for node in row {
                let count_str = match node.count {
                    Some(c) => c.to_string(),
                    None => "_".into(),
                };

                if node.value == '|' {
                    write!(f, "{}({})  ", node.value.to_string().color(Colors::BrightGreenFg), count_str)?;
                } else {
                    write!(f, "{}({})  ", node.value, count_str)?;
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
                                        Node { value: c, count: None }
                                    })
                                    .collect();
        nodes.push(line_vec);
    }

    Graph { nodes }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut graph: Graph = make_graph_from_str(&file_contents);
    let start = Instant::now();

    let paths = graph.num_of_splits(1, graph.nodes[0].len() / 2);
    println!("paths from start: {}", paths);

    let elapsed = start.elapsed();
    println!("Took: {:.2?}", elapsed);
}
