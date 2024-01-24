// implementing Dijkstra's graph traversal algorithm

//Steps:
//1. Start by adding all nodes to the set of unvisited nodes
//2. Define a start node and give that the value 0. All unvisited nodes have value max.
//3. For each node, consider the value assigned to each of its immediate neighbour
// if the value is greater than that of the current node, assign it the value of the current node
// + whatever the distance is e.g. if current node is 4, and the distance to the neighbour is 2
// , if the neighbour's value is greater than 6
// Mark the current node as visited and/or remove from the unvisited set
// The next node to visit is the one with the shortest distance
// When the unvisited nodes set is empty, the shortest path should be the sum of the minimums
// between start node and end node

use std::cmp::{Ordering};
use std::collections::BinaryHeap;

type Node = usize;
type Weight = u32;
type Graph = Vec<Vec<(Node, Weight)>>;
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct State {
    node: Node,
    cost: Weight,
}

// Implementing Ord for State so that the heap becomes a min_heap rather than a max_heap
impl State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering here
        other.cost.cmp(&self.cost)
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, start: Node, node_count: usize) -> Vec<Weight> {
    let mut dist = vec![u32::MAX; node_count];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        node: start,
        cost: 0,
    });

    while let Some(State { node, cost }) = heap.pop() {
        if cost > dist[node] {
            continue;
        }

        for &(next_node, next_cost) in &graph[node] {
            let next = State {
                node: next_node,
                cost: cost + next_cost,
            };

            if next.cost < dist[next.node] {
                heap.push(next.clone());
                dist[next.node] = next.cost;
            }
        }
    }

    dist
}

pub fn main() {
    // Example graph represented as an adjacency list
    let graph = vec![
        vec![(1, 2), (2, 4)], // Node0 is connected to Node 1 with weight 2, and to Node 2 with weight 4
        vec![(2, 1), (3, 7)], // Node 1 is connected to Node 2 with weight 1, and to Node 3 with weight 7
        vec![(3, 3)],         // Node 2 is connected to Node 3 with weight 3
        vec![],               // Node 3 has no connections
    ];

    let start_node = 0;
    let node_count = graph.len();

    let distances = dijkstra(&graph, start_node, node_count);

    println!(
        "Shortest distances from node {}: {:?}",
        start_node, distances
    );
}
