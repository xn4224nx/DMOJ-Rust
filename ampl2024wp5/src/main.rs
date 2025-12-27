/*
 * Amplitude Hackathon Winter '24 Problem 5 - WhenTaken (Year)
 * https://dmoj.ca/problem/ampl2024wp5
 */

fn main() {
    let mut buffer = String::new();

    /* Read the number of pictures. */
    std::io::stdin().read_line(&mut buffer).unwrap();

    /* Read the years of the pictures. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut pic_years = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    pic_years.sort();

    /* Read the guesses of the picture years. */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut guess_years = buffer
        .trim_end()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    guess_years.sort();

    assert_eq!(pic_years.len(), guess_years.len());

    /* Calculate the minimum penalty points. */
    let min_points = (0..pic_years.len())
        .map(|x| penalty_points(pic_years[x], guess_years[x]))
        .sum::<usize>();

    /* Calculate the maximum penalty points. */
    guess_years.reverse();
    let max_points = (0..pic_years.len())
        .map(|x| penalty_points(pic_years[x], guess_years[x]))
        .sum::<usize>();

    println!("{} {}", min_points, max_points);
}

fn penalty_points(true_year: usize, guess_year: usize) -> usize {
    return true_year.abs_diff(guess_year) * true_year.abs_diff(guess_year);
}
