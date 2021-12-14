use super::common;
use std::path::Path;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Node {
    connections: Vec<usize>,
    is_large: bool
}

impl Node {
    pub fn from_tok(tok: &[u8]) -> Node {
        let is_large = (tok[0] as char).is_uppercase();

        Node {
            connections: Vec::new(),
            is_large
        }
    }
}

fn get_or_add_node_id(tok: &[u8], id_map: &mut HashMap<String, usize>, graph: &mut Vec<Node>) -> usize {
    let id_str = String::from_iter(tok.iter().map(|x| *x as char));

    match id_map.get(&id_str) {
        Some(id) => {
            return *id;
        },
        None => {
            let id = graph.len();
            id_map.insert(id_str, id);

            let node = Node::from_tok(&tok);
            graph.push(node);

            return id;
        },
    }
}

fn tok_eq(lhs: &[u8], rhs: &[char]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    for i in 0..lhs.len() {
        if lhs[i] as char != rhs[i] {
            return false;
        }
    }

    true
}

fn is_start_node(tok: &[u8]) -> bool {
    let start_id = ['s', 't', 'a', 'r', 't'];
    tok_eq(&tok, &start_id)
}

fn is_end_node(tok: &[u8]) -> bool {
    let end_id = ['e', 'n', 'd'];
    tok_eq(&tok, &end_id)
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day12_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut ids = Vec::new();

    let (graph, start_id, end_id) = {
        let mut graph: Vec<Node> = Vec::new();

        let mut tok_to_id: HashMap<String, usize> = HashMap::new();
        let mut tok_buf = Vec::new();

        let mut start_id: Option<usize> = None;
        let mut end_id: Option<usize> = None;

        let mut cursor = 0;
        loop {
            while !(bytes[cursor] as char).is_ascii_alphabetic() {
                cursor += 1;
            }

            while (bytes[cursor] as char).is_ascii_alphabetic() {
                tok_buf.push(bytes[cursor]);
                cursor += 1;
            }

            let from_node = get_or_add_node_id(&tok_buf, &mut tok_to_id, &mut graph);
            if is_start_node(&tok_buf) {
                start_id = Some(from_node);
            } else if is_end_node(&tok_buf) {
                end_id = Some(from_node);
            }

            tok_buf.clear();
            cursor += 1;

            while cursor < bytes.len() &&
                  (bytes[cursor] as char).is_ascii_alphabetic() {
                tok_buf.push(bytes[cursor]);
                cursor += 1;
            }

            let to_node = get_or_add_node_id(&tok_buf, &mut tok_to_id, &mut graph);
            if is_start_node(&tok_buf) {
                start_id = Some(to_node);
            } else if is_end_node(&tok_buf) {
                end_id = Some(to_node);
            }

            tok_buf.clear();

            graph[from_node].connections.push(to_node);
            graph[to_node].connections.push(from_node);

            if cursor >= bytes.len() {
                break;
            }
        }

        ids.resize(graph.len(), String::default());
        for kvp in &tok_to_id {
            ids[*kvp.1] = kvp.0.clone();
        }

        (graph, start_id.unwrap(), end_id.unwrap())
    };

    let mut num_paths = 0;

    let mut closed: Vec<bool> = vec![false; graph.len()];
    closed[start_id] = true;

    // The current node and how many neighbors we need to visit
    let mut queue = VecDeque::new();
    queue.push_front((start_id, graph[start_id].connections.len()));

    let mut path = Vec::new();
    path.push(start_id);

    let mut small_seal: Option<usize> = None;

    while !queue.is_empty() {
        let top_el = queue.front_mut().unwrap();
        let cur_node_id = top_el.0;

        if (cur_node_id == end_id) || (top_el.1 == 0) {
            if cur_node_id == end_id {
                num_paths += 1;
            }

            if small_seal == Some(queue.len()) {
                small_seal = None;
            } else {
                closed[cur_node_id] = false;
            }

            path.pop();
            queue.pop_front();
        } else {
            let next_node_id = graph[cur_node_id].connections[top_el.1 - 1];
            let next_node = &graph[next_node_id];
            (*top_el).1 -= 1;

            if next_node_id == start_id {
                continue;
            }

            if !closed[next_node_id] || next_node.is_large || small_seal.is_none() {
                queue.push_front((next_node_id, next_node.connections.len()));

                if !next_node.is_large && small_seal.is_none() && closed[next_node_id] {
                    small_seal = Some(queue.len())
                }

                closed[next_node_id] = true;
                path.push(next_node_id);
            }
        }
    }

    print!("Num paths {}", num_paths);
}