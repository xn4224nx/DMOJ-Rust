/*
 * CCC '01 S3 - Strategic Bombing
 * https://dmoj.ca/problem/ccc01s3
 */

fn main() {
    let mut buffer = String::new();
    let mut roads = vec![Vec::new(); 26];

    /* Read the roads */
    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut link = buffer.chars();
        let node_0 = link.next().unwrap();
        let node_1 = link.next().unwrap();

        /* Has the exit condition been given? */
        if node_0 == '*' || node_1 == '*' {
            break;
        }

        /* Convert the node names to indexes */
        let node_0_idx = node_0 as usize - 'A' as usize;
        let node_1_idx = node_1 as usize - 'A' as usize;

        /* Save the nodes. */
        roads[node_0_idx].push(node_1_idx);
        roads[node_1_idx].push(node_0_idx);
        buffer.clear();
    }

    let mut curr_routes = vec![vec![0]];
    let mut completed_routes = Vec::new();

    /* Move from A -> B documenting all the routes. */
    while !curr_routes.is_empty() {
        let mut new_routes = Vec::new();

        /* Process each route */
        for rout in curr_routes.drain(..) {
            let last_node = rout[rout.len() - 1];

            /* Check for any completed routes. */
            if last_node == 1 {
                completed_routes.push(rout);
                continue;
            }

            /* Duplicate the routes for each possible next node. */
            for pos_nxt_node in &roads[last_node] {
                if !rout.contains(pos_nxt_node) {
                    let mut nw_route = rout.clone();
                    nw_route.push(*pos_nxt_node);
                    new_routes.push(nw_route);
                }
            }
        }

        /* Prepare for the next loop */
        curr_routes = new_routes;
    }

    /* check there is a route from A -> B. */
    if completed_routes.is_empty() {
        println!("There is no viable route from A -> B!");
        return;
    }

    /* Find the shortest route from A -> B. */
    let shrt_rt_idx = (0..completed_routes.len())
        .min_by_key(|x| completed_routes[*x].len())
        .unwrap();

    /* Keep track of the number of choke node links found. */
    let mut choke_links = 0;

    /* Iterate over the routes and check the links that they all have in common. */
    'link_check: for node_idx in 1..completed_routes[shrt_rt_idx].len() {
        let pre_node = completed_routes[shrt_rt_idx][node_idx - 1];
        let pst_node = completed_routes[shrt_rt_idx][node_idx];

        /* Check that every other route has this node link. */
        'route_check: for rt_idx in 0..completed_routes.len() {
            if rt_idx == shrt_rt_idx {
                continue;
            }

            /* Move along this route and try and find these two nodes next to each other. */
            for nd_rt_idx in 1..completed_routes[rt_idx].len() {
                if (completed_routes[rt_idx][nd_rt_idx - 1] == pre_node
                    && completed_routes[rt_idx][nd_rt_idx] == pst_node)
                    || (completed_routes[rt_idx][nd_rt_idx - 1] == pst_node
                        && completed_routes[rt_idx][nd_rt_idx] == pre_node)
                {
                    continue 'route_check;
                }
            }

            /* The link has not been found in every other route so is not a choke. */
            continue 'link_check;
        }

        /* This node link is common to all routes. */
        choke_links += 1;
        println!(
            "{}{}",
            (pre_node + 'A' as usize) as u8 as char,
            (pst_node + 'A' as usize) as u8 as char
        );
    }
    println!("There are {} disconnecting roads.", choke_links);
}
