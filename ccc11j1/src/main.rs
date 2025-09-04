/*
 * CCC '11 J1 - Which Alien?
 * https://dmoj.ca/problem/ccc11j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of antennae on the alien. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_ante = buffer.trim().parse::<usize>().unwrap();

    /* Read the number of eyes on the alien. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_eyes = buffer.trim_end().parse::<usize>().unwrap();

    /* Print the possible alien types. */
    if num_ante >= 3 && num_eyes <= 4 {
        println!("TroyMartian");
    }

    if num_ante <= 6 && num_eyes >= 2 {
        println!("VladSaturnian");
    }

    if num_ante <= 2 && num_eyes <= 3 {
        println!("GraemeMercurian");
    }
}
