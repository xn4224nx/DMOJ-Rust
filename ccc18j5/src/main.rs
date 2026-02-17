/*
 * CCC '18 J5 - Choose your own path
 * https://dmoj.ca/problem/ccc18j5
 */

fn main() {
    let mut buffer = String::new();

    /* How many pages are in the book? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_pages = buffer.trim_end().parse::<usize>().unwrap();

    let mut book = Vec::with_capacity(num_pages);

    /* Read the page links. */
    for _ in 0..num_pages {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        /* The value zero means a final page. */
        if data[0] == 0 {
            book.push(Vec::new());
        } else {
            book.push(
                (1..data.len())
                    .into_iter()
                    .map(|x| data[x] - 1)
                    .collect::<Vec<usize>>(),
            );
        }
    }
    let (all_visited, min_path) = verify_book(&book);
    println!("{}\n{}", if all_visited { "Y" } else { "N" }, min_path);
}

/// Find the shortest path and check all books.
fn verify_book(book: &Vec<Vec<usize>>) -> (bool, usize) {
    let mut visited_pages = vec![false; book.len()];
    let mut mv_count = 0;
    let mut shortest_path = usize::MAX;

    /* The book always starts on the first page. */
    let mut curr_pages = vec![0];

    /* Iterate through the book and check every page is reachable. */
    while !curr_pages.is_empty() {
        let mut nxt_pages = Vec::new();

        /* Find all the pages that the current one links to. */
        for pg_idx in curr_pages.drain(..) {
            visited_pages[pg_idx] = true;

            /* Determine if this is a final page. */
            if book[pg_idx].is_empty() {
                if mv_count < shortest_path {
                    shortest_path = mv_count;
                }

            /* Otherwise continue onto the next pages. */
            } else {
                for npg_idx in book[pg_idx].iter() {
                    if !visited_pages[*npg_idx] {
                        nxt_pages.push(*npg_idx);
                    }
                }
            }
        }

        /* Prepare for the next loop iteration. */
        curr_pages = nxt_pages;
        mv_count += 1;
    }

    return (!visited_pages.contains(&false), shortest_path + 1);
}
