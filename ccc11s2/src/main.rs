/*
 * CCC '11 S2 - Multiple Choice
 * https://dmoj.ca/problem/ccc11s2
 */

fn main() {
    let num_questions = read_num();

    /* Read the student answers */
    let student_answers: Vec<char> = (0..num_questions).map(|_| read_char()).collect();

    /* Read the answers in and calculate the score. */
    println!(
        "{}",
        (0..num_questions)
            .map(|x| read_char() == student_answers[x])
            .filter(|x| *x)
            .count()
    );
}

fn read_num() -> usize {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().parse::<usize>().unwrap();
}

fn read_char() -> char {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.chars().next().unwrap();
}
