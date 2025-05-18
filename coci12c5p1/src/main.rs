/*
 * COCI '12 Contest 5 #1 Ljestvica
 * https://dmoj.ca/problem/coci12c5p1
 */

use std::collections::HashSet;

fn main() {
    let mut c_maj_cnt = 0;
    let mut a_mnr_cnt = 0;
    let mut last_was_a_mnr = false;
    let c_maj_notes = HashSet::from(['C', 'F', 'G']);
    let a_mnr_notes = HashSet::from(['A', 'D', 'E']);

    /* Read the sheet music */
    let mut music_sheet = String::new();
    std::io::stdin().read_line(&mut music_sheet).unwrap();

    /* Determine what scale the notes belong too. */
    let mut check_next = false;
    for (ch_idx, note) in music_sheet.chars().enumerate() {
        /* Check the notes group */
        if ch_idx == 0 || check_next {
            check_next = false;

            if c_maj_notes.contains(&note) {
                c_maj_cnt += 1;
            } else if a_mnr_notes.contains(&note) {
                a_mnr_cnt += 1;
            }
        } else if note == '|' {
            check_next = true;
        }

        /* In case of a draw match the last tone */
        if ch_idx == music_sheet.len() - 2 {
            last_was_a_mnr = a_mnr_notes.contains(&note);
        }
    }

    /* Determine the overall tone of the piece of the music. */
    if c_maj_cnt > a_mnr_cnt || (c_maj_cnt == a_mnr_cnt && !last_was_a_mnr) {
        println!("C-dur")
    } else if c_maj_cnt < a_mnr_cnt || (c_maj_cnt == a_mnr_cnt && last_was_a_mnr) {
        println!("A-mol")
    }
}
