/*
 * Single Source Shortest Path
 * https://dmoj.ca/problem/sssp
 */

use std::collections::{HashMap, HashSet};

const START_NODE: usize = 0;

fn main() {
    let mut buffer = String::new();

    /* The number of nodes and edges. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dims = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Define key parts of the problem. */
    let mut unvisit_nodes: HashSet<usize> = (0..dims[0]).into_iter().collect();
    let mut node_dist: Vec<usize> = vec![usize::MAX; dims[0]];
    let mut edges: HashMap<(usize, usize), usize> = HashMap::with_capacity(dims[1]);

    /* Define the start node as being the closest. */
    node_dist[START_NODE] = 0;

    /* Read the Edge details */
    for _ in 0..dims[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let rw_edge = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Only insert if the weight is lower that previously seen. */
        if !edges.contains_key(&(rw_edge[0] - 1, rw_edge[1] - 1))
            || *edges.get(&(rw_edge[0] - 1, rw_edge[1] - 1)).unwrap() > rw_edge[2]
        {
            edges.insert((rw_edge[0] - 1, rw_edge[1] - 1), rw_edge[2]);
            edges.insert((rw_edge[1] - 1, rw_edge[0] - 1), rw_edge[2]);
        }
    }

    while !unvisit_nodes.is_empty() {
        let mut curr_node: usize = 0;
        let mut curr_min_dist = usize::MAX;

        /* Pick the unvisited node with the smallest distance from the start. */
        for u_node in unvisit_nodes.iter() {
            if node_dist[*u_node] <= curr_min_dist {
                curr_min_dist = node_dist[*u_node];
                curr_node = *u_node;
            }
        }

        /* Never use this node again. */
        unvisit_nodes.remove(&curr_node);

        /* For all unvisited neighbours of this node update their distances. */
        for u_node in unvisit_nodes.iter() {
            if edges.contains_key(&(curr_node, *u_node)) {
                let new_dist = curr_min_dist + edges.get(&(curr_node, *u_node)).unwrap();

                /* If the new distance is shorter, replace it */
                if new_dist < node_dist[*u_node] {
                    node_dist[*u_node] = new_dist;
                }
            }
        }
    }

    /* Show the smallest distance to each node */
    for node in 0..dims[0] {
        if node_dist[node] == usize::MAX {
            println!("-1");
        } else {
            println!("{}", node_dist[node]);
        }
    }
}
