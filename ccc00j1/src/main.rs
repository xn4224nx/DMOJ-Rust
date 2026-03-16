/*
 * CCC '00 J1 - Calendar
 * https://dmoj.ca/problem/ccc00j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the starting day and the length of the month. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    /* Print the header for the calendar. */
    println!("{}", cal_month(data[0], data[1]));
}

fn cal_month(start_day: usize, month_len: usize) -> String {
    let mut output = vec![String::from("Sun Mon Tue Wed Thr Fri Sat\n")];

    /* Add in the blank space before the month starts. */
    (1..start_day).for_each(|_| output.push(String::from("    ")));

    /* Add in all the dates. */
    for day in 1..=month_len {
        output.push(format!(" {: >2}", day));

        /* Nothing shoulkd be printed beyond the final date */
        if day == month_len {
            break;

        /* Detect a new row is about to start. */
        } else if (day + start_day - 1) % 7 == 0 {
            output.push(String::from("\n"));

        /* Otherwise insert a space between this day and the next one. */
        } else {
            output.push(String::from(" "));
        }
    }
    return output.join("");
}
