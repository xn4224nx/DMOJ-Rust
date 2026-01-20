/*
* CCC '10 J5 - Knight Hop
* https://dmoj.ca/problem/ccc10j5
*/

const BOARD_DIM: (usize, usize) = (8, 8);

fn main() {
    let kn_start = read_coord();
    let kn_end = read_coord();

    println!("{}", find_min_jumps(&kn_start, &kn_end));
}

fn read_coord() -> (usize, usize) {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    return (data[0] - 1, data[1] - 1);
}

/// Which of the eight possible next moves of the knight can happend on this
/// board and from this position?
pub fn next_moves(pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut nxt_mv = Vec::new();

    /* Is position 1 valid? */
    if pos.0 + 1 < BOARD_DIM.0 && pos.1 + 2 < BOARD_DIM.1 {
        nxt_mv.push((pos.0 + 1, pos.1 + 2));
    }

    /* Is position 2 valid? */
    if pos.0 + 2 < BOARD_DIM.0 && pos.1 + 1 < BOARD_DIM.1 {
        nxt_mv.push((pos.0 + 2, pos.1 + 1));
    }

    /* Is position 3 valid? */
    if pos.0 + 2 < BOARD_DIM.0 && pos.1 >= 1 {
        nxt_mv.push((pos.0 + 2, pos.1 - 1));
    }

    /* Is position 4 valid? */
    if pos.0 + 1 < BOARD_DIM.0 && pos.1 >= 2 {
        nxt_mv.push((pos.0 + 1, pos.1 - 2));
    }

    /* Is position 5 valid? */
    if pos.0 >= 1 && pos.1 >= 2 {
        nxt_mv.push((pos.0 - 1, pos.1 - 2));
    }

    /* Is position 6 valid? */
    if pos.0 >= 2 && pos.1 >= 1 {
        nxt_mv.push((pos.0 - 2, pos.1 - 1));
    }

    /* Is position 7 valid? */
    if pos.0 >= 2 && pos.1 + 1 < BOARD_DIM.1 {
        nxt_mv.push((pos.0 - 2, pos.1 + 1));
    }

    /* Is position 8 valid? */
    if pos.0 >= 1 && pos.1 + 2 < BOARD_DIM.1 {
        nxt_mv.push((pos.0 - 1, pos.1 + 2));
    }

    return nxt_mv;
}

/// Determine the minimum number of jups to get a knight from one position on
/// the board to another.
pub fn find_min_jumps(start: &(usize, usize), end: &(usize, usize)) -> usize {
    let mut visited = vec![vec![false; BOARD_DIM.0]; BOARD_DIM.1];
    let mut steps = 0;
    let mut curr_pnts = vec![*start];

    while !visited[end.0][end.1] {
        let mut nxt_pnts = Vec::new();

        /* Determine the next moves from each point. */
        for c_pnt in curr_pnts.drain(..) {
            visited[c_pnt.0][c_pnt.1] = true;

            for n_pnt in next_moves(&c_pnt) {
                if !visited[n_pnt.0][n_pnt.1] {
                    nxt_pnts.push(n_pnt);
                }
            }
        }
        curr_pnts = nxt_pnts;
        steps += 1;
    }
    return steps - 1;
}
