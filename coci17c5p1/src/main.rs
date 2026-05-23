/*
 * COCI '17 Contest 5 #1 Olivander
 * https://dmoj.ca/problem/coci17c5p1
 */

fn main() {
    let (mut item_sz, mut space_sz) = read_item_and_space();

    item_sz.sort();
    space_sz.sort();

    /* Fill the each space with largest item possible .*/
    for itm_idx in 0..item_sz.len() {
        if item_sz[itm_idx] > space_sz[itm_idx] {
            println!("NE");
            return;
        }
    }
    println!("DA");
}

/// From STDIN read the item sizes and the space sizes.
fn read_item_and_space() -> (Vec<usize>, Vec<usize>) {
    let mut buffer = String::new();

    /* Read the number of entities but don't use it. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the item sizes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let item_szs = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Read the space sizes. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let space_szs = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    assert_eq!(item_szs.len(), space_szs.len());

    return (item_szs, space_szs);
}
