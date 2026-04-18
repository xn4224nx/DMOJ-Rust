/*
 * Appleby Contest '20 P1 - Terrific Triangles
 * https://dmoj.ca/problem/ac20p1
 */

fn main() {
    let mut buffer = String::new();

    /* How many triangles will be entered on STDIN. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_triangles = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_triangles {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let mut tri_sides = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(tri_sides.len(), 3);
        tri_sides.sort();

        println!("{}", classify_triangle(&tri_sides));
    }
}

/// Determined the type of traingle based on its side lenghts. Either accute (A)
/// right angled (R) or obtuse (O).
fn classify_triangle(side_lens: &Vec<usize>) -> char {
    let small_side_sqr = side_lens[0] * side_lens[0] + side_lens[1] * side_lens[1];
    let large_side_sqr = side_lens[2] * side_lens[2];

    return if small_side_sqr < large_side_sqr {
        'O'
    } else if small_side_sqr > large_side_sqr {
        'A'
    } else {
        'R'
    };
}
