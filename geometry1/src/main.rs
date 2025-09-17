/*
 * Troubling Triangles
 * https://dmoj.ca/problem/geometry1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of triangles. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_tri = buffer.trim_end().parse::<usize>().unwrap();

    for _ in 0..num_tri {
        buffer.clear();

        /* Read the points of the triangle. */
        std::io::stdin().read_line(&mut buffer).unwrap();
        buffer.pop();
        let tri_pnts = buffer
            .splitn(6, ' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        /* Calculate the statistics for the triangle. */
        let (a, b, c) = tri_side_lens(&tri_pnts);
        let perim = tri_perimeter((a, b, c));
        let area = tri_area((a, b, c), perim);

        println!("{:.2} {:.2}", area, perim);
    }
}

/// Calculate the side lengths of a triangle using its points.
fn tri_side_lens(pnts: &Vec<f64>) -> (f64, f64, f64) {
    return (
        (((pnts[0] - pnts[2]) * (pnts[0] - pnts[2])) + ((pnts[1] - pnts[3]) * (pnts[1] - pnts[3])))
            .sqrt(),
        (((pnts[4] - pnts[2]) * (pnts[4] - pnts[2])) + ((pnts[5] - pnts[3]) * (pnts[5] - pnts[3])))
            .sqrt(),
        (((pnts[0] - pnts[4]) * (pnts[0] - pnts[4])) + ((pnts[1] - pnts[5]) * (pnts[1] - pnts[5])))
            .sqrt(),
    );
}

/// Calculate the perimeter of a triangle using its side lengths
fn tri_perimeter(sides: (f64, f64, f64)) -> f64 {
    return sides.0 + sides.1 + sides.2;
}

/// Calculate the area of a triangle using Heron's formula
fn tri_area(sides: (f64, f64, f64), perimeter: f64) -> f64 {
    return (0.5
        * perimeter
        * (0.5 * perimeter - sides.0)
        * (0.5 * perimeter - sides.1)
        * (0.5 * perimeter - sides.2))
        .sqrt();
}
