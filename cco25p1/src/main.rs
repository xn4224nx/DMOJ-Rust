/*
 * CCO '25 P1 - Asteroid Mining
 * https://dmoj.ca/problem/cco25p1
 */

#[derive(Debug)]
struct Asteroid {
    value: u64,
    mass: u64,
    val_per_mass: f64,
    picked: bool,
}

impl Asteroid {
    fn new(value: u64, mass: u64) -> Self {
        Asteroid {
            value,
            mass,
            val_per_mass: (value as f64) / (mass as f64),
            picked: false,
        }
    }
}

fn main() {
    let mut aster_field: Vec<Asteroid> = Vec::new();
    let mut buffer = String::new();

    /* Read the number of asteroids and rocket storage capacity. */
    std::io::stdin().read_line(&mut buffer).unwrap();
    let data: Vec<u64> = buffer
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let (num_aster, mass_limit) = (data[0], data[1]);

    /* Read the asteroid data from stdin. */
    for _ in 0..num_aster {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let data: Vec<u64> = buffer
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        aster_field.push(Asteroid::new(data[0], data[1]));
    }

    /* Sort the asteroid by value per mass descending. */
    aster_field.sort_by(|x, y| y.val_per_mass.total_cmp(&x.val_per_mass));

    println!("{:?}", aster_field);

    /* Fill the rocket cargo up to mass limit */
    let mut aster_value_collected = 0;
    let mut aster_mass_collected = 0;

    /* Pick the most value per mass asteroids until there is no room left. */
    loop {
        let mut aster_put_in_hold = false;

        /* Check to see if each asteroid could fit in the hold. */
        for a_idx in 0..aster_field.len() {
            if !aster_field[a_idx].picked
                && aster_field[a_idx].mass + aster_mass_collected <= mass_limit
            {
                aster_value_collected += aster_field[a_idx].value;
                aster_mass_collected += aster_field[a_idx].mass;
                aster_field[a_idx].picked = true;
                aster_put_in_hold = true;
            }
        }

        /* Stop looking if none of the remaining asteroids can fit in the hull. */
        if !aster_put_in_hold {
            break;
        }
    }

    println!("{}", aster_value_collected);
}
