/*
 * CCC '24 S1 - Hat Circle
 * https://dmoj.ca/problem/ccc24s1
 */

fn main() {
    let mut buffer = String::new();

    /* How many people will be in the circle? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_people = buffer.trim_end().parse::<usize>().unwrap();

    let mut people = Vec::with_capacity(num_people);

    /* Read the values of the people in the circle. */
    for _ in 0..num_people {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        people.push(buffer.trim_end().parse::<usize>().unwrap())
    }

    /* Determine the count of people who are oppersite eachother and have same value. */
    println!(
        "{}",
        (0..num_people / 2)
            .filter(|&x| people[x] == people[x + num_people / 2])
            .count()
            * 2
    );
}
