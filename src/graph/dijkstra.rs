use std::cmp::Reverse;
use std::collections::{HashMap, BinaryHeap};
use std::iter::Rev;
use priority_queue::PriorityQueue;

pub fn dijkstra(edges: HashMap<i32, Vec<(i32, i32)>>, start_node: i32, end_node: i32) -> i32 {
    let mut visited: HashMap<i32, bool> = HashMap::new();

    let mut min_heap = PriorityQueue::new();
    min_heap.push(start_node, Reverse(1));

    while !min_heap.is_empty() {
        let dist_info = min_heap.pop();

        match dist_info {
            None => {
                return 0
            }
            Some((node, dist)) => {
                if node == end_node {
                    return dist.0
                }

                visited.insert(node, true);
                let near_nodes = edges.get(&node);
                match near_nodes {
                    None => {
                        continue
                    }
                    Some(nears) => {
                        for (n, di) in nears {
                            if !visited.get(n).is_none() {
                                continue
                            }

                            min_heap.push(*n, Reverse(dist+di))
                        }
                    }
                }
            }
        }
    }

    return 0;
}

