use std::fs;
use std::fmt;

#[derive(Debug)]
struct Node {
    value: char,
    row: usize,
    col: usize
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Vec<Node>>,
    count: i32,
}

impl Graph {
    fn num_of_splits(&mut self, curr_row: usize, curr_col: usize) {
        if curr_row >= self.nodes.len() {
            // we've reached the end, lets return with no splits
            // println!("END \n{} {1}", self, self.count);
            return;
        }

        if curr_col >= self.nodes[0].len() {
            // out of bounds
            return;
        }

        // println!("current node: {:?} --- count so far {1}", self.nodes[curr_row][curr_col], self.count);

        match self.nodes[curr_row][curr_col].value {
            '.' => {
                self.nodes[curr_row][curr_col].value = '|';

                // if (self.nodes[curr_row - 1][curr_col].value != '|') {
                //     self.count += 1;
                // }

                self.num_of_splits(curr_row + 1, curr_col);
                // println!("RETURNING, NOW COUNT IS {} at {curr_row}-{curr_col}", self.count);
            },
            '|' => {
                // do nothing, go no further
            }, 
            '^' => {
                if (self.nodes[curr_row][curr_col-1].value != '|'){
                    self.count += 1;
                }

                self.num_of_splits(curr_row, curr_col + 1);

                if (self.nodes[curr_row][curr_col+1].value != '|'){
                    self.count += 1;
                }
                self.num_of_splits(curr_row, curr_col - 1);
            },
            _ => {
                // do nothing
            }
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.nodes {
            for node in row {
                write!(f, "{}", node.value)?;
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
                                        Node { value: c, row: i, col: j }
                                    })
                                    .collect();
        nodes.push(line_vec);
    }

    Graph { nodes, count: 0 }
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut graph: Graph = make_graph_from_str(&file_contents);

    // println!("{:?}", graph);
    graph.num_of_splits(1, graph.nodes[0].len()/2);
    println!("{}", graph.count);
}
