/*
 * COCI '06 Regional #1 Bard
 * https://dmoj.ca/problem/crci06p1
 */

use std::collections::HashSet;

fn main() {
    let bard_idx = 0;
    let mut buffer = String::new();

    /* How many villagers are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_villagers = buffer.trim().parse::<usize>().unwrap();

    /* How many evenings are there? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_evenings = buffer.trim().parse::<usize>().unwrap();

    /* Record what songs each villager knows. */
    let mut known_songs: Vec<HashSet<usize>> = vec![HashSet::new(); num_villagers];

    /* Read the attendances of each evening. */
    for eve_idx in 0..num_evenings {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let eve_vill_idxs: Vec<usize> = buffer
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();

        /* If the Bard is present they teach everyone a single song. */
        if eve_vill_idxs.contains(&bard_idx) {
            for v_idx in eve_vill_idxs.iter() {
                known_songs[*v_idx].insert(eve_idx);
            }

        /* Otherwise everyone shares the songs they know */
        } else {
            let mut present_songs: HashSet<usize> = HashSet::new();

            /* What songs does everyone know. */
            for v_idx in eve_vill_idxs.iter() {
                present_songs.extend(&known_songs[*v_idx]);
            }

            /* Now all the present visitors know this set of songs. */
            for v_idx in eve_vill_idxs.iter() {
                known_songs[*v_idx].extend(&present_songs);
            }
        }
    }

    /* What villagers know the same number of songs as the bard. */
    for v_idx in 0..num_villagers {
        if known_songs[v_idx].len() == known_songs[bard_idx].len() {
            println!("{}", v_idx + 1);
        }
    }
}
