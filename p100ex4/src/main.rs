/*
 * BlueBook - Cross Country
 * https://dmoj.ca/problem/p100ex4
 */

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!(
        "{}",
        match buffer.as_str() {
            "MG\n" => "midget girls",
            "MB\n" => "midget boys",
            "JG\n" => "junior girls",
            "JB\n" => "junior boys",
            "SG\n" => "senior girls",
            "SB\n" => "senior boys",
            _ => "invalid code",
        }
    );
}
