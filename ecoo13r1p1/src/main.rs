/*
 * ECOO '13 R1 P1 - Take a Number
 * https://dmoj.ca/problem/ecoo13r1p1
 */

fn main() {
    let mut buffer = String::new();
    let mut num_in_line = 0;
    let mut num_late = 0;

    /* Read the starting number. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut ticket_num = buffer.trim().parse::<u16>().unwrap();

    /* Read and process the commands. */
    while buffer != "EOF\n" {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        /* Simulate the ticket machine being refilled. */
        if ticket_num > 999 {
            ticket_num = 1
        }

        /* Process the ticket. */
        if buffer == "TAKE\n" {
            ticket_num += 1;
            num_in_line += 1;
            num_late += 1;
        } else if buffer == "SERVE\n" {
            num_in_line -= 1;
        } else if buffer == "CLOSE\n" {
            println!("{} {} {}", num_late, num_in_line, ticket_num);
            num_in_line = 0;
            num_late = 0;
        }
    }
}
