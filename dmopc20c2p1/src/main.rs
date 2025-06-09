/*
 * DMOPC '20 Contest 2 P1 - Laugh Graphs
 * https://dmoj.ca/problem/dmopc20c2p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the raw financial changes. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let changes: Vec<char> = buffer.trim().chars().collect();

    /* Parse the financial changes and determine the position of the line */
    let mut line_chs: Vec<char> = Vec::new();
    let mut all_pos: Vec<i64> = Vec::new();
    let mut curr_pos = 0;
    for idx in 0..changes.len() {
        if changes[idx] == '^' {
            all_pos.push(curr_pos);
            curr_pos += 1;
            line_chs.push('/');
        } else if changes[idx] == 'v' {
            curr_pos -= 1;
            all_pos.push(curr_pos);
            line_chs.push('\\');
        } else {
            all_pos.push(curr_pos);
            line_chs.push('_');
        }
    }

    /* Normalise the positions. */
    let max_pos = all_pos.iter().max().unwrap();
    let min_pos = all_pos.iter().min().unwrap();
    let positions: Vec<usize> = all_pos.iter().map(|x| (max_pos - x) as usize).collect();

    /* Determine the dimensions of the graph. */
    let width = changes.len();
    let height = (1 + max_pos - min_pos) as usize;

    /* Fill the graph with empty squares */
    let mut graph = vec![vec!['.'; width]; height];

    /* Draw the line on graph. */
    for l_idx in 0..width {
        graph[positions[l_idx]][l_idx] = line_chs[l_idx];
    }

    /* Show the graph. */
    for h_idx in 0..height {
        for w_idx in 0..width {
            print!("{}", graph[h_idx][w_idx]);
        }
        println!();
    }
}
