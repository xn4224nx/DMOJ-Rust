/*
 * Amplitude Hackathon Winter '24 Problem 3 - Killer Queen
 * https://dmoj.ca/problem/ampl2024wp3
 * https://en.wikipedia.org/wiki/Partition_problem
 */

const TEAM_SIZE: usize = 10;

fn main() {
    let mut buffer = String::new();

    /* Read the drone skill levels. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let drone_skls = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the queen skill levels. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let queen_skls = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("{}", stacked_diff_finder(&queen_skls, &drone_skls));
}

fn stacked_diff_finder(queen_skills: &Vec<usize>, drone_skills: &Vec<usize>) -> usize {
    assert_eq!(queen_skills.len(), TEAM_SIZE);
    assert_eq!(drone_skills.len(), TEAM_SIZE);
    let mut smallest_diff = usize::MAX;

    /* Systematically choose every combination of players to be queens. */
    for q0_idx in 0..(drone_skills.len() - 1) {
        for q1_idx in (q0_idx + 1)..drone_skills.len() {
            /* Extract the values of all the drones. */
            let mut aval_drones = (0..drone_skills.len())
                .filter(|x| x != &q0_idx && x != &q1_idx)
                .map(|x| drone_skills[x])
                .collect::<Vec<usize>>();

            /* Calculate the total drone skills. */
            let total_drone_skill = aval_drones.iter().sum::<usize>();

            /* Iterate over every possible team composition. */
            for d0_idx in 0..(aval_drones.len() - 3) {
                for d1_idx in 1..(aval_drones.len() - 2) {
                    for d2_idx in 2..(aval_drones.len() - 1) {
                        for d3_idx in 3..aval_drones.len() {
                            let team_0_skill = aval_drones[d0_idx]
                                + aval_drones[d1_idx]
                                + aval_drones[d2_idx]
                                + aval_drones[d3_idx];
                            let team_1_skill = total_drone_skill - team_0_skill;

                            /* Calculate the difference betweem the teams. */
                            let min_diff = (queen_skills[q0_idx] + team_0_skill)
                                .abs_diff(queen_skills[q1_idx] + team_1_skill);

                            /* See if a new smallest difference has been found. */
                            if min_diff < smallest_diff {
                                smallest_diff = min_diff;
                            }
                        }
                    }
                }
            }
        }
    }
    return smallest_diff;
}

/*
    /* Systematically choose every combination of players to be queens. */
    for q0_idx in 0..(drone_skills.len() - 1) {
        for q1_idx in (q0_idx + 1)..drone_skills.len() {


            let mut team0_skl = queen_skills[q0_idx];
            let mut team1_skl = queen_skills[q1_idx];


            let mut rev_aval_drones = VecDeque::from(aval_drones);

            /* Assign the drones to try and get the teams to the same size. */
            let mut team0_len = rev_aval_drones.len() / 2;
            let mut team1_len = rev_aval_drones.len() / 2;

            while !rev_aval_drones.is_empty() {
                if team0_len > 0 && team1_len > 0 {
                    if team0_skl < team1_skl {
                        team0_skl += rev_aval_drones.pop_front().unwrap();
                        team0_skl += rev_aval_drones.pop_back().unwrap();
                        team0_len -= 2;
                    } else {
                        team1_skl += rev_aval_drones.pop_front().unwrap();
                        team1_skl += rev_aval_drones.pop_back().unwrap();
                        team1_len -= 2;
                    }
                } else if team0_len > 0 {
                    team0_skl += rev_aval_drones.pop_front().unwrap();
                    team0_skl += rev_aval_drones.pop_back().unwrap();
                    team0_len -= 2;
                } else {
                    team1_skl += rev_aval_drones.pop_front().unwrap();
                    team1_skl += rev_aval_drones.pop_back().unwrap();
                    team1_len -= 2;
                }
            }

            /* See if a new smallest difference has been found. */
            let new_diff = team0_skl.abs_diff(team1_skl);
            if new_diff < smallest_diff {
                smallest_diff = new_diff;
            }
        }
    }

*/
