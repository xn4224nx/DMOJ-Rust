/*
* CCC '23 J2 - Chili Peppers
* https://dmoj.ca/problem/ccc23j2
*/

fn main() {
    let mut buffer = String::new();
    let mut heat = 0;

    /* How many peppers are there going to be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_peppers = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_peppers {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* How does the heat change as every pepper is added? */
        heat += match buffer.as_str() {
            "Poblano\n" => 1500,
            "Mirasol\n" => 6000,
            "Serrano\n" => 15500,
            "Cayenne\n" => 40000,
            "Thai\n" => 75000,
            "Habanero\n" => 125000,
            _ => 0,
        };
    }

    println!("{}", heat);
}
