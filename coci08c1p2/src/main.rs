/*
 * COCI '08 Contest 1 #2 Ptice
 * https://dmoj.ca/problem/coci08c1p2
 */

fn main() {
    let names = vec!["Adrian", "Bruno", "Goran"];
    let guesses = vec![
        vec!['A', 'B', 'C'],
        vec!['B', 'A', 'B', 'C'],
        vec!['C', 'C', 'A', 'A', 'B', 'B'],
    ];
    let mut correct = vec![0; guesses.len()];

    /* Read the number of questions. */
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_q = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    /* Read the answers and convert to vector of chars. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let answers: Vec<char> = buffer.trim().chars().collect();

    assert_eq!(answers.len(), num_q);

    /* Generate the three boys answers and count the number they get right. */
    for q_idx in 0..num_q {
        for stu_idx in 0..guesses.len() {
            if answers[q_idx] == guesses[stu_idx][q_idx % guesses[stu_idx].len()] {
                correct[stu_idx] += 1;
            }
        }
    }

    /* Determine the winning score */
    let max_score = correct.iter().max().unwrap();
    println!("{}", max_score);

    /* Print the names of the winners */
    for stu_idx in 0..guesses.len() {
        if correct[stu_idx] == *max_score {
            println!("{}", names[stu_idx]);
        }
    }
}
