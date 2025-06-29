/*
 * CCC '09 J2 - Old Fishin' Hole
 * https://dmoj.ca/problem/ccc09j2
 */

fn main() {
    let mut fish_pnts = Vec::new();
    let mut buffer = String::new();
    let mut catch_combs = 0;

    /* Read the fish points. */
    for _ in 0..4 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        fish_pnts.push(buffer.trim().parse::<u32>().unwrap());
    }
    let (brw_cost, pike_cost, yell_cost, limit) =
        (fish_pnts[0], fish_pnts[1], fish_pnts[2], fish_pnts[3]);

    for num_brw in 0..=limit.div_ceil(brw_cost) {
        for num_pike in 0..=limit.div_ceil(pike_cost) {
            for num_yell in 0..=limit.div_ceil(yell_cost) {
                let total_score = brw_cost * num_brw + pike_cost * num_pike + yell_cost * num_yell;
                if total_score <= limit && total_score > 0 {
                    println!(
                        "{} Brown Trout, {} Northern Pike, {} Yellow Pickerel",
                        num_brw, num_pike, num_yell
                    );
                    catch_combs += 1;
                }
            }
        }
    }
    println!("Number of ways to catch fish: {}", catch_combs);
}
