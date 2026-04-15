/*
 * WC '95 P4 - Wildcard
 * https://dmoj.ca/problem/wc95p4
 */

const ANY_MATCH: char = '?';
const ZERO_MORE_MATCH: char = '*';

fn main() {
    let (words, queries) = read_data();
    show_all_matches(&words, &queries);
}

/// Show the matches for each query from the supllied dictionary of words.
fn show_all_matches(word_dict: &Vec<Vec<char>>, all_queries: &Vec<Vec<char>>) {
    for q_idx in 0..all_queries.len() {
        let wrd_matches = (0..word_dict.len())
            .filter(|x| query_match(&word_dict[*x], &all_queries[q_idx]))
            .collect::<Vec<usize>>();

        println!(
            "{}",
            if wrd_matches.is_empty() {
                String::from("NO MATCH")
            } else {
                wrd_matches
                    .into_iter()
                    .map(|x| word_dict[x].iter().collect())
                    .collect::<Vec<String>>()
                    .join(", ")
            }
        );
    }
}

/// Determine if a word matches a query
fn query_match(word: &Vec<char>, query: &Vec<char>) -> bool {
    let mut viable_comp_idxs = vec![(0, 0)];

    /* Ensure that the word can fit in the pattern. */
    if word.len() < query.iter().filter(|x| **x != ZERO_MORE_MATCH).count() {
        return false;
    }

    /* Determined if there are ways the word and the query could match. */
    while !viable_comp_idxs.is_empty() {
        let mut new_comps = Vec::new();

        /* Evaluate each comparison idx pair. */
        for (q_idx, w_idx) in viable_comp_idxs.drain(..) {
            if q_idx < query.len() && w_idx < word.len() {
                /* Handle branching posibilities from ZERO_MORE_MATCH. */
                if query[q_idx] == ZERO_MORE_MATCH {
                    /* The ZERO_MORE_MATCH represents no chars in the word. */
                    if q_idx + 1 < query.len() {
                        new_comps.push((q_idx + 1, w_idx));
                    }

                    /* The ZERO_MORE_MATCH represents 1+ chars in the word. */
                    if w_idx + 1 < word.len() {
                        new_comps.push((q_idx, w_idx + 1));
                    }

                    /* One letter in the word matches one in the query. */
                } else if query[q_idx] == ANY_MATCH || query[q_idx] == word[w_idx] {
                    new_comps.push((q_idx + 1, w_idx + 1));
                }
            }

            /* Check for a solution. */
            if q_idx == query.len() - 1 && w_idx >= word.len() - 1 {
                return true;
            }
        }
        viable_comp_idxs = new_comps;
    }
    return false;
}

/// From STDIN read and parse the dictionary words and the queries.
fn read_data() -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let num_queries = 5;
    let mut buffer = String::new();

    /* How many words are in the dictionary? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let dict_len = buffer.trim_end().parse::<usize>().unwrap();

    let mut word_dict = Vec::with_capacity(dict_len);
    let mut queries = Vec::with_capacity(num_queries);

    /* Read the dictionary. */
    for _ in 0..dict_len {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        word_dict.push(buffer.trim().chars().collect());
    }

    /* Read the queries. */
    for _ in 0..num_queries {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        queries.push(buffer.trim().chars().collect());
    }
    return (word_dict, queries);
}
