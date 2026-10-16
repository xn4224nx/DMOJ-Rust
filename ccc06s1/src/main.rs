/*
 * CCC '06 S1 - Maternity
 * https://dmoj.ca/problem/ccc06s1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the parents genes. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mother_genes = buffer.trim_end().chars().collect::<Vec<char>>();
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let father_genes = buffer.trim_end().chars().collect::<Vec<char>>();

    /* How many babies will there be? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_babies = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the babies genes and determined parenthood. */
    for _ in 0..num_babies {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let baby_genes = buffer.trim_end().chars().collect::<Vec<char>>();

        println!(
            "{}",
            if is_related(&mother_genes, &father_genes, &baby_genes) {
                "Possible baby."
            } else {
                "Not their baby!"
            }
        );
    }
}

/// Determine if a baby is related to a mother and a father based on the genes.
fn is_related(m_genes: &Vec<char>, f_genes: &Vec<char>, b_genes: &Vec<char>) -> bool {
    assert_eq!(m_genes.len(), b_genes.len() * 2);
    assert_eq!(f_genes.len(), b_genes.len() * 2);

    /* Check that all genes in the baby can be found in the parents. */
    for b_g_idx in 0..b_genes.len() {
        /* Recessive genes need to be present in both parents. */
        if b_genes[b_g_idx].is_ascii_lowercase() {
            if (b_genes[b_g_idx] != m_genes[2 * b_g_idx]
                && b_genes[b_g_idx] != m_genes[2 * b_g_idx + 1])
                || (b_genes[b_g_idx] != f_genes[2 * b_g_idx]
                    && b_genes[b_g_idx] != f_genes[2 * b_g_idx + 1])
            {
                return false;
            }

        /* If the babies gene is uppercase check one of the parents has it. */
        } else {
            if b_genes[b_g_idx] != m_genes[2 * b_g_idx]
                && b_genes[b_g_idx] != m_genes[2 * b_g_idx + 1]
                && b_genes[b_g_idx] != f_genes[2 * b_g_idx]
                && b_genes[b_g_idx] != f_genes[2 * b_g_idx + 1]
            {
                return false;
            }
        }
    }
    return true;
}
