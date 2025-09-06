/*
 * CCC '08 J1 - Body Mass Index
 * https://dmoj.ca/problem/ccc08j1
 */

fn main() {
    let mut buffer = String::new();

    /* Read the patients weight. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let weight = buffer
        .trim_end()
        .parse::<f64>()
        .expect("Weight couldn't be parsed!");

    /* Read the patients height. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let height = buffer
        .trim_end()
        .parse::<f64>()
        .expect("Height couldn't be parsed!");

    /* Calculate the patients BMI. */
    let bmi = weight / (height * height);

    if bmi > 25.0 {
        println!("Overweight");
    } else if bmi < 18.5 {
        println!("Underweight");
    } else {
        println!("Normal weight");
    }
}
