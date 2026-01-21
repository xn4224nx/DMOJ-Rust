/*
 * CCC '05 J3 - Returning Home
 * https://dmoj.ca/problem/ccc05j3
 */

fn main() {
    let mut buffer = String::new();
    let mut instructs = Vec::new();

    /* Read the instructions. */
    while buffer != "SCHOOL\n" {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        instructs.push(buffer.clone());
    }

    /* Remove the exit command. */
    instructs.pop();

    /* Process the instructions backwards and print in a human readable format. */
    while !instructs.is_empty() {
        let direct = match instructs.pop().unwrap().as_str().trim() {
            "L" => "RIGHT",
            "R" => "LEFT",
            _ => panic!("Unknown direction!"),
        };

        println!(
            "{}",
            if instructs.is_empty() {
                format!("Turn {} into your HOME.", direct)
            } else {
                format!(
                    "Turn {} onto {} street.",
                    direct,
                    instructs.pop().unwrap().trim()
                )
            }
        );
    }
}
