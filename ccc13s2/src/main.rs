/*
 * CCC '13 S2 - Bridge Transport
 * https://dmoj.ca/problem/ccc13s2
 */

const MAX_NUM_CARS: usize = 4;

fn main() {
    let mut buffer = String::new();

    /* What is the weight capacity of the bridge? */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let brdg_weight_capac = buffer.trim_end().parse::<usize>().unwrap();

    /* How many cars are in the train? */
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let num_cars = buffer.trim_end().parse::<usize>().unwrap();
    let mut train_car_weights = Vec::with_capacity(num_cars);

    /* Read the train car weights. */
    for _ in 0..num_cars {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        train_car_weights.push(buffer.trim_end().parse::<usize>().unwrap());
    }

    let mut win_s_idx = 0;
    let mut win_e_idx = std::cmp::min(train_car_weights.len(), MAX_NUM_CARS);

    while win_s_idx < train_car_weights.len() {
        let mut curr_win_weight: usize = train_car_weights[win_s_idx..win_e_idx].iter().sum();

        /* Does the current window break the bridge. */
        if curr_win_weight > brdg_weight_capac {
            /* Shrink the window till it passes the weight limit. */
            while curr_win_weight > brdg_weight_capac {
                win_e_idx -= 1;
                curr_win_weight = train_car_weights[win_s_idx..win_e_idx].iter().sum();
            }
            break;
        }

        /* Move onto the next window. */
        win_s_idx += 1;
        win_e_idx += 1;

        /* Ensure the window is always valid. */
        if win_e_idx > train_car_weights.len() {
            win_e_idx = train_car_weights.len();
        }
    }
    println!("{}", win_e_idx);
}
