/*
 * COCI '18 Contest 2 #1 Preokret
 * https://dmoj.ca/problem/coci18c2p1
 */

fn main() {
    let (
        mut half_time_points,
        mut turn_arounds,
        mut team_a_pts,
        mut team_b_pts,
        mut a_idx,
        mut b_idx,
    ) = (0, 0, 0, 0, 0, 0);
    let (mut a_winning, mut b_winning) = (false, false);

    /* Read the times for team A. */
    let team_a_times = read_team_times();
    let team_b_times = read_team_times();

    /* Simulate the game being played. */
    for time_idx in 1..=2880 {
        /* Does A score? */
        if a_idx < team_a_times.len() && team_a_times[a_idx] == time_idx {
            team_a_pts += 1;
            a_idx += 1;
        }

        /* Does B score? */
        if b_idx < team_b_times.len() && team_b_times[b_idx] == time_idx {
            team_b_pts += 1;
            b_idx += 1;
        }

        /* Save the points at half time */
        if time_idx == 1440 {
            half_time_points = team_a_pts + team_b_pts;
        }

        /* Detect a turn around for A. */
        if team_a_pts > team_b_pts && b_winning && !a_winning {
            b_winning = false;
            a_winning = true;
            turn_arounds += 1;

        /* Detect a turn around for B. */
        } else if team_a_pts < team_b_pts && !b_winning && a_winning {
            b_winning = true;
            a_winning = false;
            turn_arounds += 1;

        /* B takes the lead at the start of the game. */
        } else if !b_winning && !a_winning && team_a_pts < team_b_pts {
            b_winning = true;

        /* A takes the lead at the start of the game. */
        } else if !b_winning && !a_winning && team_a_pts > team_b_pts {
            a_winning = true;
        }
    }
    println!("{}\n{}", half_time_points, turn_arounds);
}

fn read_team_times() -> Vec<u32> {
    let mut buffer = String::new();

    /* Read the total team points. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let points = buffer.trim().parse::<usize>().unwrap();
    let mut times = Vec::with_capacity(points);

    /* Read the times for the team. */
    for _ in 0..points {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        times.push(buffer.trim().parse::<u32>().unwrap());
    }
    return times;
}
