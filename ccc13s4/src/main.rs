/*
 * CCC '13 S4 - Who is Taller?
 * https://dmoj.ca/problem/ccc13s4
 */

fn main() {
    let mut buffer = String::new();

    /* Read & parse the graph metadata. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let metadata = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Keep a record of the edges */
    let mut more_than = vec![vec![false; metadata[0]]; metadata[0]];

    /* Read the edge data for the graph. */
    for _ in 0..metadata[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let edge = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<usize>>();
        more_than[edge[0]][edge[1]] = true;
    }

    /* Read the comparison nodes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let comp_nodes = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<usize>>();

    /* Determine if the nodes are connected. */
    println!(
        "{}",
        if are_nodes_conn(comp_nodes[0], comp_nodes[1], &more_than) {
            "yes"
        } else if are_nodes_conn(comp_nodes[1], comp_nodes[0], &more_than) {
            "no"
        } else {
            "unknown"
        }
    );
}

/// With a network of unidirecional edges determined if two nodes are connected?
fn are_nodes_conn(start_node: usize, end_node: usize, uni_edges: &Vec<Vec<bool>>) -> bool {
    let mut curr_nodes = vec![start_node];
    let mut seen_nodes = vec![false; uni_edges.len()];
    seen_nodes[start_node] = true;

    while !curr_nodes.is_empty() {
        let mut nxt_nodes = Vec::new();

        /* Collect the unvisited nodes connected to each current node. */
        for c_node in curr_nodes.drain(..) {
            for n_idx in 0..uni_edges.len() {
                if !seen_nodes[n_idx] && uni_edges[c_node][n_idx] {
                    seen_nodes[n_idx] = true;
                    nxt_nodes.push(n_idx);

                    /* Check to see if the destination has been reached. */
                    if n_idx == end_node {
                        return true;
                    }
                }
            }
        }
        curr_nodes = nxt_nodes;
    }

    /* At this point there is no way to connect the nodes via the supplied edges. */
    return false;
}
