use super::common;
use std::path::Path;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct Step {
    cost: usize,
    node: usize,
}

impl Step {
    pub fn new(node: usize, cost: usize) -> Step {
        Step {cost, node}
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Flip the order of comparison so we 
            .then_with(|| self.node.cmp(&other.node)) // get a min-sorted heap.
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day15_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());
    
    let (grid, width, height) = common::parse_grid(&bytes);

    fn get_cost(node: usize, graph: &Vec<u8>) -> usize {
        graph[node] as usize
    }
    
    let start_node = 0;
    let end_node = grid.len() - 1;

    let mut queue = BinaryHeap::new();
    let mut prev_nodes: Vec<Option<usize>> = vec![None; grid.len()];
    let mut node_costs = vec![usize::MAX; grid.len()];
    
    queue.push(Step::new(start_node, 0));
    prev_nodes[start_node] = None;
    node_costs[start_node] = get_cost(start_node, &grid);

    while !queue.is_empty() {
        let current_step = queue.pop().unwrap();
        let current_node = current_step.node;

        let neighbors = common::get_cross_taps(current_node, width, height);
        for next in neighbors {
            if next.is_none() {
                continue;
            }

            let next_node = next.unwrap();
            let new_cost = node_costs[current_node] + get_cost(next_node, &grid);
            
            if (node_costs[next_node] == usize::MAX) || (new_cost < node_costs[next_node]) {
                node_costs[next_node] = new_cost;
                prev_nodes[next_node] = Some(current_node);
                queue.push(Step::new(next_node, new_cost));
            }  
        }
    }

    let mut path = Vec::new();
    let mut found_node = end_node;
    loop {
        path.push((get_cost(found_node, &grid), common::get_grid_xy(found_node, width, height)));

        match prev_nodes[found_node] {
            Some(node) => found_node = node,
            None => break,
        }
    }

    path.reverse();

    let mut total: usize = 0;
    for el in &path[1..(path.len())] { // Exclude end from cost since we dont leave it.
        // print!("({}, {}, c={}), ", el.1.0, el.1.1, el.0);
        total += el.0;
    }

    println!("Total path cost: {}", total);
}