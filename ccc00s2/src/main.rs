/*
 * CCC '00 S2 - Babbling Brooks
 * https://dmoj.ca/problem/ccc00s2
 */

use std::collections::VecDeque;

fn main() {
    let (mut streams, instructs) = read_data();

    /* Iterate over the instructions and change the streams */
    for insr_idx in 0..instructs.len() {
        let instr = &instructs[insr_idx];

        /* Merge the stream and the one to its right. */
        if instr.len() == 1 {
            streams[instr[0]] += streams[instr[0] - 1];
            streams.remove(instr[0] - 1);

        /* The split stream and the percentage that flows to the left */
        } else if instr.len() == 2 {
            let l_stream = (instr[1] as f64) * streams[instr[0] - 1] / 100.0;
            let r_stream = streams[instr[0] - 1] - l_stream;

            /* Set the left stream */
            streams[instr[0] - 1] = l_stream;

            /* Insert the right stream */
            streams.insert(instr[0], r_stream);
        }
    }

    /* Print the streams rounded to the closest integer. */
    println!(
        "{}",
        streams
            .into_iter()
            .map(|x| x.round().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

/// Read the original stream and splits and joins
fn read_data() -> (VecDeque<f64>, Vec<Vec<usize>>) {
    let mut buffer = String::new();
    let mut all_instruc = Vec::new();
    let mut tmp_instruc = Vec::new();

    /* Read the number of streams. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_streams = buffer.trim().parse::<usize>().unwrap();
    let mut streams = VecDeque::with_capacity(num_streams);

    /* Read the stream flows */
    for _ in 0..num_streams {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        streams.push_back(buffer.trim().parse::<f64>().unwrap());
    }

    /* Read the splits and joins. */
    let mut join_cnt = 0;
    let mut splt_cnt = 0;
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let num = buffer.trim().parse::<usize>().unwrap();

        /* Detect a change in type. */
        if join_cnt == 0 && splt_cnt == 0 {
            /* Save the instructions */
            if !tmp_instruc.is_empty() {
                all_instruc.push(tmp_instruc.clone());
                tmp_instruc.clear();
            }
            match num {
                77 => break,
                88 => join_cnt += 1,
                99 => splt_cnt += 2,
                _ => panic!("Unknown command '{}'", num),
            };

        /* Save split infomation. */
        } else if splt_cnt > 0 {
            tmp_instruc.push(num);
            splt_cnt -= 1;

        /* Save the join infomation. */
        } else if join_cnt > 0 {
            tmp_instruc.push(num);
            join_cnt -= 1;
        } else {
            panic!("Unknown state reached from '{}' ", num);
        }
    }
    return (streams, all_instruc);
}
