/*
 * DMOPC '17 Contest 5 P1 - IOI 101
 * https://dmoj.ca/problem/dmopc17c5p1
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Replace characters */
    for pass_char in buffer.chars() {
        print!(
            "{}",
            match pass_char {
                '0' => 'O',
                '1' => 'l',
                '3' => 'E',
                '4' => 'A',
                '5' => 'S',
                '6' => 'G',
                '8' => 'B',
                '9' => 'g',
                _ => pass_char,
            }
        );
    }
}
