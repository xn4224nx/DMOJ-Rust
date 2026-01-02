/*
 * Ternary Search Tree
 * https://dmoj.ca/problem/ternarysearchtree
 */

const EMPTY_BRANCH: usize = usize::MAX;

struct Node {
    value: usize,
    left_idx: usize,
    mid_idx: usize,
    right_idx: usize,
}

impl Node {
    fn new(value: usize) -> Self {
        Node {
            value,
            left_idx: EMPTY_BRANCH,
            mid_idx: EMPTY_BRANCH,
            right_idx: EMPTY_BRANCH,
        }
    }
}

fn main() {
    let mut buffer = String::new();

    /* How big will the tree be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_vals = buffer.trim_end().parse::<usize>().unwrap();
    let mut tree = Vec::with_capacity(num_vals);

    for _ in 0..num_vals {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let entry = buffer.trim_end().parse::<usize>().unwrap();

        /* Initalalise an empty tree. */
        if tree.is_empty() {
            tree.push(Node::new(entry));
            continue;
        }

        /* Add a value into the tree. */
        let mut node_idx = 0;
        loop {
            /* Move left. */
            if tree[node_idx].value > entry {
                if tree[node_idx].left_idx == EMPTY_BRANCH {
                    tree.push(Node::new(entry));
                    tree[node_idx].left_idx = tree.len() - 1;
                    break;
                } else {
                    node_idx = tree[node_idx].left_idx;
                }

            /* Move right. */
            } else if tree[node_idx].value < entry {
                if tree[node_idx].right_idx == EMPTY_BRANCH {
                    tree.push(Node::new(entry));
                    tree[node_idx].right_idx = tree.len() - 1;
                    break;
                } else {
                    node_idx = tree[node_idx].right_idx;
                }

            /* Move to the middle. */
            } else {
                if tree[node_idx].mid_idx == EMPTY_BRANCH {
                    tree.push(Node::new(entry));
                    tree[node_idx].mid_idx = tree.len() - 1;
                    break;
                } else {
                    node_idx = tree[node_idx].mid_idx;
                }
            }
        }
        /* Output the value of the parent of the node that was just inserted. */
        println!("{}", tree[node_idx].value);
    }
}
