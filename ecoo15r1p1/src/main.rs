/*
 * ECOO '15 R1 P1 - When You Eat Your Smarties
 * https://dmoj.ca/problem/ecoo15r1p1
 */

fn main() {
    let mut remaining_boxes = 10;
    let mut buffer = String::new();

    /* There are variable number of boxes. */
    while remaining_boxes > 0 {
        let mut sweets: Vec<u32> = vec![0; 8];

        /* Read the colour names. */
        loop {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            match buffer.as_str() {
                "red\n" => sweets[0] += 1,
                "orange\n" => sweets[1] += 1,
                "blue\n" => sweets[2] += 1,
                "green\n" => sweets[3] += 1,
                "yellow\n" => sweets[4] += 1,
                "pink\n" => sweets[5] += 1,
                "violet\n" => sweets[6] += 1,
                "brown\n" => sweets[7] += 1,
                "end of box\n" => break,
                _ => panic!("{} is not a valid colour", buffer),
            };
        }

        /* Red sweets are eaten individually. */
        let mut time = sweets[0] * 16;

        /* Other sweets are eaten in handfuls of 7 */
        for sw_idx in 1..sweets.len() {
            time += 13 * sweets[sw_idx].div_ceil(7)
        }

        /* The number of seconds it would require to eat all the sweets. */
        println!("{}", time);
        remaining_boxes -= 1;
    }
}
