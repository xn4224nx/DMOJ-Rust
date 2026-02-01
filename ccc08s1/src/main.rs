/*
 * CCC '08 S1 - It's Cold Here!
 * https://dmoj.ca/problem/ccc08s1
 */

fn main() {
    let mut buffer = String::new();
    let mut min_temp = i32::MAX;
    let mut min_temp_city = String::new();

    /* Read city temperatures until Waterloo. */
    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data = buffer.trim().split_whitespace().collect::<Vec<&str>>();

        /* What is the city temperature? */
        let city_temp = data[1].parse::<i32>().unwrap();

        /* Has a new record low found? */
        if city_temp < min_temp {
            min_temp = city_temp;
            min_temp_city = data[0].to_string();
        }

        /* is this the final city? */
        if data[0] == "Waterloo" {
            break;
        }
        buffer.clear();
    }
    println!("{}", min_temp_city);
}
