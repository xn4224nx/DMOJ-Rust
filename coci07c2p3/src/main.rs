/*
 * COCI '07 Contest 2 #3 Prva
 * https://dmoj.ca/problem/coci07c2p3
 */

fn main() {
    let min_word_len = 2;
    let max_word_len = 20;
    let blck = '#';
    let mut buffer = String::with_capacity(max_word_len);

    /* Read the two dimensions of the crossword. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dims = buffer
        .trim_end()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut crossword: Vec<Vec<char>> = vec![Vec::with_capacity(dims[1]); dims[0]];
    let mut blocked: Vec<(usize, usize)> = Vec::new();
    let mut words: Vec<String> = Vec::new();

    /* Read the crossword itself. */
    for row_idx in 0..dims[0] {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        for (col_idx, letter) in buffer.trim_end().chars().enumerate() {
            crossword[row_idx].push(letter);

            /* Keep track of the blocked off squares */
            if letter == blck {
                blocked.push((row_idx, col_idx));
            }
        }
    }

    /* Extract the words that could start from the top edge and go down. */
    buffer.clear();
    for col_idx in 0..dims[1] {
        if crossword[0][col_idx] == blck {
            continue;
        }

        /* Read characters until the edge of the crossword is found or a block. */
        for chr_idx in 0..dims[0] {
            if crossword[chr_idx][col_idx] == blck {
                break;
            } else {
                buffer.push(crossword[chr_idx][col_idx])
            }
        }

        /* determine if this word should be saved */
        if buffer.len() >= min_word_len {
            words.push(buffer.clone());
        }
        buffer.clear();
    }

    /* Extract words that could start from the left edge and go across */
    for row_idx in 0..dims[0] {
        if crossword[row_idx][0] == blck {
            continue;
        }

        /* Read characters until the edge of the crossword is found or a block. */
        for chr_idx in 0..dims[1] {
            if crossword[row_idx][chr_idx] == blck {
                break;
            } else {
                buffer.push(crossword[row_idx][chr_idx])
            }
        }

        /* determine if this word should be saved */
        if buffer.len() >= min_word_len {
            words.push(buffer.clone());
        }
        buffer.clear();
    }

    /* Extract words that could start right or down of a block. */
    for (blk_row, blk_col) in blocked.into_iter() {
        /* Extract the word going down. */
        for char_idx in blk_row + 1..dims[0] {
            if crossword[char_idx][blk_col] == blck {
                break;
            } else {
                buffer.push(crossword[char_idx][blk_col])
            }
        }

        /* Determine if this word should be saved. */
        if buffer.len() >= min_word_len {
            words.push(buffer.clone());
        }
        buffer.clear();

        /* Extract the word going to the right. */
        for char_idx in blk_col + 1..dims[1] {
            if crossword[blk_row][char_idx] == blck {
                break;
            } else {
                buffer.push(crossword[blk_row][char_idx])
            }
        }

        /* Determine if this word should be saved. */
        if buffer.len() >= min_word_len {
            words.push(buffer.clone());
        }
        buffer.clear();
    }

    /* Output the lexicographically smallest word in the crossword. */
    words.sort();
    println!("{}", words[0]);
}
