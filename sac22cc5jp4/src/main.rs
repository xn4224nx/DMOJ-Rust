/*
 * SAC '22 Code Challenge 5 Junior P4 - Course Requirements
 * https://dmoj.ca/problem/sac22cc5jp4
 */

use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();

    /* How many courses are there? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_courses = buffer.trim_end().parse::<usize>().unwrap();

    let mut course_reqs: Vec<HashSet<usize>> = Vec::with_capacity(num_courses);

    /* Read the course prerequisites. */
    for _ in 0..num_courses {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        course_reqs.push(
            buffer
                .trim_end()
                .split(' ')
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap() - 1)
                .collect::<HashSet<usize>>(),
        );
    }

    /* Keep a record of courses that have been completed. */
    let mut study_order: Vec<usize> = Vec::with_capacity(num_courses);
    let mut uncomp_courses: HashSet<usize> = (0..num_courses).collect();

    /* Determine the order courses must be completed in. */
    while study_order.len() < num_courses {
        let mut rm_courses = Vec::new();

        /* Find the next course that has its prerequisites met. */
        for course_idx in uncomp_courses.iter() {
            if course_reqs[*course_idx].is_empty() {
                rm_courses.push(*course_idx);
            }
        }

        /* Remove completed courses in order. */
        rm_courses.sort();
        for rm_idx in rm_courses.drain(..) {
            study_order.push(rm_idx + 1);
            uncomp_courses.remove(&rm_idx);

            /* Remove this course from the other course requirements. */
            for course_idx in 0..num_courses {
                course_reqs[course_idx].remove(&rm_idx);
            }
        }
    }

    /* Show the final study order. */
    println!(
        "{}",
        study_order
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
