/*
 * ECOO '12 R1 P2 - Decoding DNA
 * https://dmoj.ca/problem/ecoo12r1p2
 */

fn main() {
    let termin_len = 6;
    let promo = vec!['T', 'A', 'T', 'A', 'A', 'T'];
    let mut buffer = String::new();

    for problem_idx in 0..5 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Convert the dna strand to a vector of chars. */
        let dna: Vec<char> = buffer.trim().chars().collect();

        /* Find the promotor. */
        let mut promo_start = 0;
        'promo_search: for d_idx in 0..dna.len() - promo.len() {
            /* Check the upcoming genes to see if they match. */
            for p_idx in 0..promo.len() {
                if promo[p_idx] != dna[d_idx + p_idx] {
                    continue 'promo_search;
                }
            }

            /* If everything matches record when the promo ends. */
            promo_start = d_idx;
            break 'promo_search;
        }

        println!("{}", promo_start);

        /* Find the terminator, it will at least 10 genes from the promo. */
        let mut term_start = 0;
        'term_search: for d_idx in (10 + promo_start)..(dna.len() - termin_len) {
            let search_term = reverse_flip(&dna[d_idx..d_idx + termin_len]);

            /* See if any of the rest of the genes are the search term. */
            'term_check: for s_idx in (d_idx + termin_len)..(dna.len() - termin_len) {
                for n_idx in 0..search_term.len() {
                    if search_term[n_idx] != dna[s_idx + n_idx] {
                        continue 'term_check;
                    }
                }



                /* A match has been found. */
                term_start = d_idx;


                println!("{}", &dna[d_idx..d_idx + termin_len].iter().collect::<String>());
                println!("{}", &dna[s_idx..s_idx + termin_len].iter().collect::<String>());
                println!("{}", &dna[(10 + promo_start)..term_start].iter().collect::<String>());


                break 'term_search;
            }
        }



        /* Create the transcription unit. */
        println!(
            "{}: {}\n",
            problem_idx + 1,
            convert_2_rna(&dna[(10 + promo_start)..term_start])
                .into_iter()
                .collect::<String>()
        );
    }
}


fn reverse_flip(gene_seq: &[char]) -> Vec<char> {
    return gene_seq
        .iter()
        .map(|x| match x {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("Unknown gene {}", x),
        })
        .rev()
        .collect();
}

fn convert_2_rna(gene_seq: &[char]) -> Vec<char> {
    return gene_seq
        .iter()
        .map(|x| match x {
            'A' => 'U',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => panic!("Unknown gene {}", x),
        })
        .collect();
}


/*

AGTCTTCAAGGGGATTCCCAGG TATATAATGCAG ATCGCGACGAAATAT  CGGGCG GGATCCATACCGACCCAGC  CGCCCG A




CGGGCG
CGCCCG
ATCGCGACGAAATA T


*/







































