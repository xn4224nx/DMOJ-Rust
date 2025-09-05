/*
 * IOI '09 P5 - Garage
 * https://dmoj.ca/problem/ioi09p5
 */

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let mut revenue = 0;
    let mut buffer = String::new();

    /* Read the number of spaces and cars. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    /* Read the details of the problem */
    let pspace_cost: HashMap<Reverse<usize>, usize> = read_data(data[0])
        .into_iter()
        .enumerate()
        .map(|(x, y)| (Reverse(x), y))
        .collect();
    let car_weights: Vec<usize> = read_data(data[1]);
    let car_flux: Vec<i32> = read_data(2 * data[1]);

    /* Record what space each car is in. */
    let mut carpark: HashMap<usize, Reverse<usize>> = HashMap::new();

    /* Record the indexes of the cars waiting to park. */
    let mut carpark_queue: VecDeque<usize> = VecDeque::new();

    /* Record the free spaces in the carpark. */
    let mut free_spaces: BinaryHeap<Reverse<usize>> = (0..data[0]).map(|x| Reverse(x)).collect();

    /* Simultate the arrival and departure of cars */
    for flux in car_flux.into_iter() {
        let car_idx = flux.abs() as usize - 1;

        /* A Car arrives */
        if flux > 0 {
            /* Is there a space for the car to park in? */
            if !free_spaces.is_empty() {
                let free_space_idx = free_spaces.pop().unwrap();

                /* Park the car. */
                carpark.insert(car_idx, free_space_idx);

                /* Pay for the parking space. */
                revenue += pspace_cost.get(&free_space_idx).unwrap() * car_weights[car_idx];

            /* Otherwise add it to the end of the queue */
            } else {
                carpark_queue.push_back(car_idx)
            }

        /* A car leaves */
        } else {
            /* A car park space frees up */
            let freed_space_idx = carpark.remove(&car_idx).unwrap();
            free_spaces.push(freed_space_idx);

            /* If there are any cars queuing they take the spot. */
            if !carpark_queue.is_empty() {
                let free_space_idx = free_spaces.pop().unwrap();
                let new_car_idx = carpark_queue.pop_front().unwrap();

                /* Park the car. */
                carpark.insert(new_car_idx, free_space_idx);

                /* Pay for the parking space. */
                revenue += pspace_cost.get(&free_space_idx).unwrap() * car_weights[new_car_idx];
            }
        }
    }
    println!("{}", revenue);
}

/// Read a predetermined number of values from STDIN seperated by newlines
fn read_data<T: FromStr>(num_lines: usize) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut data: Vec<T> = Vec::with_capacity(num_lines);
    let mut buffer = String::new();

    for _ in 0..num_lines {
        std::io::stdin().read_line(&mut buffer).unwrap();
        data.push(buffer.trim().parse::<T>().unwrap());
        buffer.clear();
    }
    return data;
}
