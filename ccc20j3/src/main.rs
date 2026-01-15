/*
 * CCC '20 J3 - Art
 * https://dmoj.ca/problem/ccc20j3
 */

const DIMS: usize = 2;
const PADDING: usize = 1;
const DELIM_PAT: char = ',';

fn main() {
    let mut buffer = String::new();
    let mut furth_reaches = vec![vec![usize::MAX, 0]; DIMS];

    /* How many points will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_points = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the points, line by line. */
    for _ in 0..num_points {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let point = buffer
            .trim_end()
            .split(DELIM_PAT)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        assert_eq!(point.len(), DIMS);

        /* Determine the furthest reaches in each dimension. */
        for d_idx in 0..DIMS {
            if point[d_idx] > furth_reaches[d_idx][1] {
                furth_reaches[d_idx][1] = point[d_idx];
            }
            if point[d_idx] < furth_reaches[d_idx][0] {
                furth_reaches[d_idx][0] = point[d_idx];
            }
        }
    }

    /* Show the furthest reaches of the points.  */
    for reac_idx in 0..furth_reaches[0].len() {
        println!(
            "{}",
            (0..DIMS)
                .into_iter()
                .map(|x| furth_reaches[x][reac_idx])
                .map(|x| match reac_idx {
                    0 => x.saturating_sub(PADDING),
                    1 => x.saturating_add(PADDING),
                    _ => panic!("Unsupported turning point!"),
                })
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(&DELIM_PAT.to_string())
        );
    }
}
