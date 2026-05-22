/*
 * Arcadia Computing Contest 2 P1 - Rejected
 * https://dmoj.ca/problem/ahscc2p1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the competition variables. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let comp_defs = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    buffer.clear();

    /* Read the player scores. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let ply_scrs = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!(
        "{}",
        if does_player_pass(&comp_defs, &ply_scrs) {
            "MASTER"
        } else {
            "REJECTED AGAIN"
        }
    )
}

/// Has the player passed the compatition?
fn does_player_pass(comp_rules: &Vec<usize>, player_scrs: &Vec<usize>) -> bool {
    let max_scr = 100;

    /* Ensure the total score is high enough. */
    if player_scrs.iter().sum::<usize>() < comp_rules[0] {
        return false;
    }

    /* Ensure the max score has been reached on enough problems. */
    if player_scrs.iter().filter(|x| x >= &&max_scr).count() < comp_rules[1] {
        return false;
    }

    /* Ensure that there enough non-zero answers. */
    if player_scrs.iter().filter(|x| x >= &&0).count() < comp_rules[2] {
        return false;
    }

    /* Otherwise they have passed. */
    return true;
}
