/*
 * CCC '03 J1 - Trident
 * https://dmoj.ca/problem/ccc03j1
 */

const TRI_MARK: char = '*';
const SPCE_PXL: char = ' ';

fn main() {
    let mut buffer = String::new();

    /* Read the trident specifications. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let specs = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    print!("{}", forge_trident(specs[0], specs[1], specs[2]));
}

fn forge_trident(tri_val: usize, spc_val: usize, hnd_val: usize) -> String {
    let mut trident = String::new();

    /* Create the three points. */
    for _ in 0..tri_val {
        trident.push(TRI_MARK);
        for _ in 0..spc_val {
            trident.push(SPCE_PXL);
        }
        trident.push(TRI_MARK);
        for _ in 0..spc_val {
            trident.push(SPCE_PXL);
        }
        trident.push(TRI_MARK);
        trident.push('\n');
    }

    /* Create the bar connecting the three points. */
    for _ in 0..(3 + spc_val * 2) {
        trident.push(TRI_MARK);
    }
    trident.push('\n');

    /* Create the handle */
    for _ in 0..hnd_val {
        for _ in 0..=spc_val {
            trident.push(SPCE_PXL);
        }
        trident.push(TRI_MARK);
        trident.push('\n');
    }

    return trident;
}
