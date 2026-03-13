/*
 * VM7WC '16 #3 Silver - Can Shahir even get there?!
 * https://dmoj.ca/problem/vmss7wc16c3p2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the graph infomation. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let grph_info = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* What if the end and the beginning are the same node? */
    if grph_info[2] == grph_info[3] {
        println!("GO SHAHIR!");
        return;
    }

    /* Store the node each other node links to. */
    let mut nodes = vec![Vec::new(); grph_info[0]];

    /* Keep a record of if each node has been visted. */
    let mut visted_nodes = vec![false; grph_info[0]];

    /* Read the edge data. */
    for _ in 0..grph_info[1] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let edge = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* Save the link between the two nodes. */
        nodes[edge[0] - 1].push(edge[1] - 1);
        nodes[edge[1] - 1].push(edge[0] - 1);
    }

    /* See if there is a link between A and B */
    let mut curr_nodes = vec![grph_info[2] - 1];

    loop {
        let mut nxt_nodes = Vec::new();

        /* for each current node get all the unvisted nodes it links to. */
        for c_node in curr_nodes.drain(..) {
            visted_nodes[c_node] = true;

            for n_node in nodes[c_node].iter() {
                if !visted_nodes[*n_node] {
                    nxt_nodes.push(*n_node);

                    /* See if the end has been reached. */
                    if *n_node == grph_info[3] - 1 {
                        println!("GO SHAHIR!");
                        return;
                    }
                }
            }
        }

        /* If there is no next nodes the end cannot be reached. */
        if nxt_nodes.is_empty() {
            println!("NO SHAHIR!");
            return;
        } else {
            curr_nodes = nxt_nodes;
        };
    }
}
