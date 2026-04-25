/*
 * DMOPC '20 Contest 1 P1 - Victor Gets Quizzed
 * https://dmoj.ca/problem/dmopc20c1p1
 */

fn main() {
    let mut buffer = String::new();

    /* Now many quizes will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_quiz = buffer.trim_end().parse::<usize>().unwrap();

    /* Read and mark the quizes. */
    for _ in 0..num_quiz {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Test results are dependant on subject presence. */
        println!(
            "{}",
            if buffer.contains("C") && buffer.contains("M") {
                "NEGATIVE MARKS"
            } else if buffer.contains("C") || buffer.contains("M") {
                "FAIL"
            } else {
                "PASS"
            }
        );
    }
}
