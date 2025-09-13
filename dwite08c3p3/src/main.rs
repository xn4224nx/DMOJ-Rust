/*
 * DWITE '08 R3 #3 - Unicorns and Teaspoons
 * https://dmoj.ca/problem/dwite08c3p3
 */

fn main() {
    let mut buffer = String::new();
    let unit_converter = vec![
        vec![1.0, 5.0, 20.0, 40.0, 160.0],
        vec![0.2, 1.0, 4.0, 8.0, 32.0],
        vec![0.05, 0.25, 1.0, 2.0, 8.0],
        vec![0.025, 0.125, 0.5, 1.0, 4.0],
        vec![0.00625, 0.03125, 0.125, 0.25, 1.0],
    ];

    /* Read the values to be converted. */
    for _ in 0..5 {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data: Vec<&str> = buffer.trim_end().split_whitespace().collect();

        /* What is the value in the original units. */
        let value = data[0].parse::<f64>().unwrap();

        /* What is the conversion factor. */
        let conver_fac = unit_converter[unit_to_index(data[3])][unit_to_index(data[1])];

        /* What is the value in the new units */
        println!("{}", (value * conver_fac).round() as usize)
    }
}

fn unit_to_index(unit: &str) -> usize {
    match unit {
        "oz" => 0,
        "gill" => 1,
        "pt" => 2,
        "qt" => 3,
        "gal" => 4,
        _ => panic!("Unknown unit"),
    }
}
