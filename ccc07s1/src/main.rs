/*
 * CCC '07 S1 - Federal Voting Age
 * https://dmoj.ca/problem/ccc07s1
 */

fn main() {
    let mut buffer = String::new();

    /* How many dates will there be? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_dates = buffer.trim_end().parse::<usize>().unwrap();

    /* Read the dates and determine if they are eligable. */
    for _ in 0..num_dates {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();

        println!(
            "{}",
            match eligable_voter(&buffer) {
                true => "Yes",
                false => "No",
            }
        );
    }
}

fn eligable_voter(rw_birthday: &str) -> bool {
    let date = rw_birthday
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Check the year */
    if date[0] < 1989 {
        return true;
    } else if date[0] > 1989 {
        return false;
    }

    /* Check the month. */
    if date[1] < 2 {
        return true;
    } else if date[1] > 2 {
        return false;
    }

    /* Check the day. */
    if date[2] <= 27 {
        return true;
    } else {
        return false;
    }
}
