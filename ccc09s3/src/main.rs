/*
 * CCC '09 S3 - Degrees Of Separation
 * https://dmoj.ca/problem/ccc09s3
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    let mut nodes = vec![
        HashSet::from([5]),
        HashSet::from([5]),
        HashSet::from([3, 4, 5, 14]),
        HashSet::from([2, 4, 5]),
        HashSet::from([2, 3, 5]),
        HashSet::from([0, 1, 2, 3, 4, 6]),
        HashSet::from([5, 7]),
        HashSet::from([6, 8]),
        HashSet::from([7, 9, 11]),
        HashSet::from([8, 10]),
        HashSet::from([9, 11]),
        HashSet::from([8, 10, 12]),
        HashSet::from([11, 13, 14]),
        HashSet::from([12]),
        HashSet::from([2, 12]),
        HashSet::from([16, 17]),
        HashSet::from([15, 17]),
        HashSet::from([15, 16]),
    ];

    'cmd_exe: loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let cmd = buffer.chars().next().unwrap();

        /* Has the exit command been given? */
        if cmd == 'q' {
            return;
        }

        /* Read and parse the preceeding node indexes. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if cmd == 'i' || cmd == 'd' || cmd == 's' {
            std::io::stdin().read_line(&mut buffer).unwrap();
        }
        let idxs = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<usize>>();

        /* Ensure that the mentioned indexes exist. */
        let max_idx = *idxs.iter().max().unwrap();
        while nodes.len() <= max_idx {
            nodes.push(HashSet::new());
        }

        /* Execute the command */
        match cmd {
            'i' => {
                /* Insert an edge between two nodes. */
                nodes[idxs[0]].insert(idxs[1]);
                nodes[idxs[1]].insert(idxs[0]);
            }

            'd' => {
                /* Remove an edge between two nodes. */
                nodes[idxs[0]].remove(&idxs[1]);
                nodes[idxs[1]].remove(&idxs[0]);
            }

            'n' => {
                /* How many edges does a node have? */
                println!("{}", nodes[idxs[0]].len());
            }

            'f' => {
                /* How many nodes are the nodes connected to this one connected to. */
                let mut node_of_node: HashSet<usize> = HashSet::new();

                /* Collect nodes linked to nodes, linked to the original. */
                for n_idx in nodes[idxs[0]].iter() {
                    node_of_node.extend(nodes[*n_idx].iter());
                }

                /* Remove nodes linked to the original. */
                for n_idx in nodes[idxs[0]].iter() {
                    node_of_node.remove(n_idx);
                }

                /* Remove the original node. */
                node_of_node.remove(&idxs[0]);

                println!("{}", node_of_node.len());
            }

            's' => {
                /* What is the shortest path between two nodes? */
                let mut curr_nodes = HashSet::from([idxs[0]]);
                let mut steps = 0;
                let mut seen_nodes = HashSet::new();

                while !curr_nodes.is_empty() {
                    let mut nxt_nodes = HashSet::new();
                    steps += 1;

                    /* Find all the nodes connected to this one. */
                    for c_node in curr_nodes.drain() {
                        seen_nodes.insert(c_node);

                        for n_node in nodes[c_node].iter() {
                            if *n_node == idxs[1] {
                                println!("{}", steps);
                                continue 'cmd_exe;
                            } else if !seen_nodes.contains(n_node) {
                                nxt_nodes.insert(*n_node);
                            }
                        }
                    }

                    /* Continue onto the next step. */
                    curr_nodes = nxt_nodes;
                }

                /* The search has ended and the destination has not been found. */
                println!("Not connected");
            }
            _ => panic!("Unknown command!"),
        };
    }
}
