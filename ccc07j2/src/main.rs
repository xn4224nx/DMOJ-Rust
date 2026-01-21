/*
 * CCC '07 J2 - I Speak TXTMSG
 * https://dmoj.ca/problem/ccc07j2
 */

fn main() {
    let mut buffer = String::new();

    while buffer != "TTYL\n".to_string() {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        print!(
            "{}",
            match buffer.as_str() {
                "CU\n" => "see you\n",
                ":-)\n" => "I'm happy\n",
                ":-(\n" => "I'm unhappy\n",
                ";-)\n" => "wink\n",
                ":-P\n" => "stick out my tongue\n",
                "(~.~)\n" => "sleepy\n",
                "TA\n" => "totally awesome\n",
                "CCC\n" => "Canadian Computing Competition\n",
                "CUZ\n" => "because\n",
                "TY\n" => "thank-you\n",
                "YW\n" => "you're welcome\n",
                "TTYL\n" => "talk to you later\n",
                _ => &buffer,
            }
        );
    }
}
