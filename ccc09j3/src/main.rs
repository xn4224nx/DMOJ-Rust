/*
 * CCC '09 J3 - Good Times
 * https://dmoj.ca/problem/ccc09j3
 */

fn main() {
    let location_offsets = vec![
        (0, 0, "Ottawa"),
        (21, 0, "Victoria"),
        (22, 0, "Edmonton"),
        (23, 0, "Winnipeg"),
        (0, 0, "Toronto"),
        (1, 0, "Halifax"),
        (1, 30, "St. John's"),
    ];

    let mut buffer = String::new();

    /* Read the time in Ottawa. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let raw_time = buffer.trim_end().parse::<usize>().unwrap();

    /* Convert the raw time to the numbers of minutes and hours. */
    let orig_mins = raw_time % 100;
    let orig_hours = (raw_time - orig_mins) / 100;

    /* Print the time in various locations. */
    for (h_off, m_off, name) in location_offsets.into_iter() {
        let mins = (orig_mins + m_off) % 60;
        let hours = ((orig_hours + h_off + (orig_mins + m_off) / 60) % 24) * 100;
        println!("{} in {}", hours + mins, name);
    }
}
