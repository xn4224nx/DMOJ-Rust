/*
 * CCC '22 J3 - Harp Tuning
 * https://dmoj.ca/problem/ccc22j3
 */

fn main() {
    let mut buffer = String::new();
    let mut tun_srt = String::new();
    let mut tun_num = String::new();

    /* Read the line of compressed instructions. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Expand the instructions. */
    for ins_ch in buffer.chars() {
        if ins_ch.is_ascii_uppercase() || ins_ch == '\n' {
            tun_srt.push(ins_ch);

            /* Print the previous command's digit. */
            if !tun_num.is_empty() {
                println!("{}", tun_num.parse::<usize>().unwrap());
                tun_num.clear();
            }
        } else if ins_ch == '-' || ins_ch == '+' {
            print!(
                "{} {} ",
                tun_srt,
                match ins_ch {
                    '+' => "tighten",
                    '-' => "loosen",
                    _ => panic!("Unknown direction."),
                }
            );
            tun_srt.clear();
        } else if ins_ch.is_ascii_digit() {
            tun_num.push(ins_ch);
        }
    }
}
