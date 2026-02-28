/*
 * DWITE '08 R3 #1 - ASCII Rhombus
 * https://dmoj.ca/problem/dwite08c3p1
 */

const NUM_INPUTS: usize = 5;

fn main() {
    let mut buffer = String::new();

    /* Read the sizes of the rhombus's. */
    for _ in 0..NUM_INPUTS {
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Show the rhombus's. */
        print!(
            "{}",
            draw_rhombus(buffer.trim_end().parse::<usize>().unwrap())
        );
        buffer.clear();
    }
}

fn draw_rhombus(rmb_sz: usize) -> String {
    return match rmb_sz {
        1 => String::from("#\n"),
        3 => String::from(".#.\n###\n.#.\n"),
        5 => String::from("..#..\n.###.\n#####\n.###.\n..#..\n"),
        7 => String::from(concat!(
            "...#...\n",
            "..###..\n",
            ".#####.\n",
            "#######\n",
            ".#####.\n",
            "..###..\n",
            "...#...\n"
        )),
        _ => panic!("Unsupported size!"),
    };
}
