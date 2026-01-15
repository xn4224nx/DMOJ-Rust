/*
 * CCC '01 J2 - Mod Inverse
 * https://dmoj.ca/problem/ccc01j2
 */

fn main() {
    let mut buffer = String::new();

    /* Read the two variables. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let vars = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!(
        "{}",
        match find_mod_inverse(vars[0], vars[1]) {
            Ok(value) => value.to_string(),
            Err(err) => err,
        }
    );
}

/// Test each possible value and try and find a number that is the mod inverse.
fn find_mod_inverse(x_var: usize, m_var: usize) -> Result<usize, String> {
    for n_var in 1..m_var {
        if (n_var * x_var) % m_var == 1 {
            return Ok(n_var);
        }
    }
    return Err(String::from("No such integer exists."));
}
