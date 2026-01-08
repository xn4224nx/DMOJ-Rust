/*
 * CCC '21 J2 - Silent Auction
 * https://dmoj.ca/problem/ccc21j2
 */

fn main() {
    let mut buffer0 = String::new();
    let mut buffer1 = String::new();

    let mut max_bid = 0;
    let mut max_bidder = String::new();

    /* Read the number of bids. */
    std::io::stdin().read_line(&mut buffer0).unwrap();
    let num_bids = buffer0.trim_end().parse::<usize>().unwrap();

    /* Read the bids and find the largest. */
    for _ in 0..num_bids {
        buffer0.clear();
        std::io::stdin().read_line(&mut buffer0).unwrap();

        /* Read the persons bid. */
        buffer1.clear();
        std::io::stdin().read_line(&mut buffer1).unwrap();
        let bid = buffer1.trim_end().parse::<usize>().unwrap();

        /* Is this bid the highest so far? */
        if bid > max_bid {
            max_bid = bid;
            max_bidder = buffer0.clone();
        }
    }

    /* Who has one the bid? */
    print!("{}", max_bidder);
}
