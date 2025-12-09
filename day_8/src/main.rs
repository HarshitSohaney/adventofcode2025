use std::fs;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/**
 * [b1, b2, b5]
 * 
 * [b3, b4]
 * 
 * [b4, b2, 3]
 */
#[derive(Debug, Hash, Eq, PartialEq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct JunctionBox {
    coordinates: Coordinate,
    connected_to: Option<usize> // what set are you a part of?
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct BoxPair {
    i: usize,
    j: usize,
    dist: i64
}

impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        // For BinaryHeap, this gives a max-heap by distance
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for BoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn create_boxes(data: &str) -> Vec<JunctionBox> {
    let mut boxes: Vec<JunctionBox> = Vec::new();

    for line in data.lines() {
         let curr_box: [i64; 3] = line
            .split(',')
            .filter_map(|c| c.parse::<i64>().ok())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        boxes.push(JunctionBox {
            coordinates: Coordinate {
                x: curr_box[0],
                y: curr_box[1],
                z: curr_box[2],
            },
            connected_to: None,
        });
    }

    boxes
}

fn find_distances(boxes: &[JunctionBox]) -> BinaryHeap<BoxPair> {
    let mut heap: BinaryHeap<BoxPair> = BinaryHeap::new();
    for i in 0..boxes.len() {
        for j in (i+1)..boxes.len() {
            let b1 = &boxes[i];
            let b2 = &boxes[j];

            let dx = b1.coordinates.x - b2.coordinates.x;
            let dy = b1.coordinates.y - b2.coordinates.y;
            let dz = b1.coordinates.z - b2.coordinates.z;

            let distance = dx.pow(2) + dy.pow(2) + dz.pow(2);

            // use -distance to make it a min heap
            heap.push(BoxPair { i, j, dist: -distance });
        }
    }

    heap
}

fn merge_sets(circuits: &mut Vec<HashSet<usize>>, boxes: &mut [JunctionBox], a: usize, b: usize) {
    if a == b { return; }

    let (keep, merge) = if a < b { (a, b) } else { (b, a) };

    let (left, right) = circuits.split_at_mut(merge);
    let keep_set = &mut left[keep];
    let merge_set = &mut right[0];

    let members: Vec<usize> = merge_set.iter().copied().collect();

    keep_set.extend(merge_set.iter().copied());

    for member in members {
        boxes[member].connected_to = Some(keep);
    }

    merge_set.clear();
}

fn main() {
    let file_contents = fs::read_to_string("day_8/example.txt").expect("Failed to read input");
    let mut boxes: Vec<JunctionBox> = create_boxes(&file_contents);
    let mut heap: BinaryHeap<BoxPair> = find_distances(&boxes);
    let mut added_boxes: HashSet<usize> = HashSet::new();

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut last_i = 0;
    let mut last_j = 0;

    while heap.len() > 1 && added_boxes.len() < boxes.len() {
        let smallest_pair: BoxPair = heap.pop().unwrap();
        let i = smallest_pair.i;
        let j = smallest_pair.j;

        // Track the last pair for final calculation
        last_i = i;
        last_j = j;

        let ci = boxes[i].connected_to;
        let cj = boxes[j].connected_to;

        match (ci, cj) {
            (None, None) => {
                // Neither box is in a circuit yet, create a new circuit.
                let mut set = HashSet::new();
                set.insert(i);
                set.insert(j);

                let circuit_idx = circuits.len();
                circuits.push(set);

                boxes[i].connected_to = Some(circuit_idx);
                boxes[j].connected_to = Some(circuit_idx);

                added_boxes.insert(i);
                added_boxes.insert(j);
            }
            (Some(ci_idx), None) => {
                // j joins i's circuit.
                circuits[ci_idx].insert(j);
                boxes[j].connected_to = Some(ci_idx);
                added_boxes.insert(j);
            }
            (None, Some(cj_idx)) => {
                // i joins j's circuit.
                circuits[cj_idx].insert(i);
                boxes[i].connected_to = Some(cj_idx);
                added_boxes.insert(i);
            }
            (Some(ci_idx), Some(cj_idx)) => {
                // Both already in some circuit
                merge_sets(&mut circuits, &mut boxes, ci_idx, cj_idx);
            }
        }
    }
    println!("{}", boxes[last_i].coordinates.x * boxes[last_j].coordinates.x);
    let mut circuits_vec: Vec<&HashSet<usize>> = circuits
        .iter()
        .filter(|set| !set.is_empty())
        .collect();

    circuits_vec.sort_by_key(|set| std::cmp::Reverse(set.len()));

    let mut res = 1;

    // for i in 0..3 {
    //     res *= circuits_vec[i].len();
    // }

    // println!("{res}");
}
