/*
 * IOI '04 P5 - Farmer
 * https://dmoj.ca/problem/ioi04p5
 */

fn main() {
    let tree_stats = read_nums(3);
    let mut tree_capacity = tree_stats[0];

    /* Read the cypress tree field sizes */
    let mut fields = read_nums(tree_stats[1]);

    /* Read the cypress tree strip sizes. */
    let mut strips = read_nums(tree_stats[2]);

    /* Calculate the total number of cypress trees in each group type. */
    let total_field_trees = fields.iter().sum::<u32>();
    let total_strips_trees = strips.iter().sum::<u32>();


    /* Can the trees be satisifed by fields alone. */
    if tree_capacity == total_field_trees {
        println!("{}", total_field_trees);


    /*  */
    } else if   total_field_trees   <  tree_capacity {



    } else {


    }


    /* Sort the tree groups largest to smallest. */
    fields.sort_by(|a, b| b.cmp(a));
    strips.sort_by(|a, b| b.cmp(a));

    /* Record the ammount of olives gained. */
    let mut olive_trees = 0;

    /* Use as many fields of cypress as possible. */
    for f_idx in 0..fields.len() {
        /* See if we can afford this field */
        if fields[f_idx] <= tree_capacity {
            tree_capacity -= fields[f_idx];
            olive_trees += fields[f_idx];

            /* Check to see if we can no longer pick groups of trees. */
            if tree_capacity == 0 {
                println!("{}", olive_trees);
                return;
            }
        }
    }

    /* Make up the remaining capacity with strips of cypress trees. */
    for s_idx in 0..strips.len() {
        /* See if we can afford this strip of trees */
        if strips[s_idx] <= tree_capacity {
            tree_capacity -= strips[s_idx];
            olive_trees += strips[s_idx] - 1;

            /* Check to see if we can no longer pick groups of trees. */
            if tree_capacity == 0 {
                println!("{}", olive_trees);
                return;
            }
        }
    }
    println!("{}", olive_trees);
}

fn read_nums(num_count: u32) -> Vec<u32> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer
        .splitn(num_count as usize, ' ')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();
}
