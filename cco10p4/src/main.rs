/*
 * CCO '10 P4 - Computer Purchase Return
 * https://dmoj.ca/problem/cco10p4
 */

fn main() {
    let prob = ComputerBuild::new();

    /* Stop early if the problem can't be solved. */
    println!(
        "{}",
        if !prob.is_viable() {
            -1
        } else {
            prob.brute_best_computer_value()
        }
    );
}

struct ComputerBuild {
    budget: usize,
    components: Vec<Vec<(usize, usize)>>,
}

impl ComputerBuild {
    /// Read the component details, types and the computers budget from STDIN.
    fn new() -> Self {
        let mut buffer = String::new();

        /* How many types of components are required? */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let types_of_compo = buffer.trim_end().parse::<usize>().unwrap();

        let mut components = vec![Vec::new(); types_of_compo];

        /* How many components are there? */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let compo_count = buffer.trim_end().parse::<usize>().unwrap();

        /* Read the component data. */
        for _ in 0..compo_count {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let data = buffer
                .trim_end()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            /* Seperate the components by type */
            components[data[2] - 1].push((data[0], data[1]));
        }

        /* Read the budget. */
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let budget = buffer.trim_end().parse::<usize>().unwrap();

        /* Remove components that cannot be used. */
        components = (0..types_of_compo)
            .map(|y| components[y].drain(..).filter(|x| x.0 < budget).collect())
            .collect();

        return ComputerBuild { components, budget };
    }

    /// Can this combination of components & budget build any computer at all?
    fn is_viable(&self) -> bool {
        /* If there is not at least one component of each type the computer cannot be built. */
        if (0..self.components.len()).any(|x| self.components[x].is_empty()) {
            return false;
        }

        /* Can the computer be built with the cheapest components within the budget? */
        if (0..self.components.len())
            .map(|x| self.components[x].iter().map(|y| y.0).min().unwrap())
            .sum::<usize>()
            > self.budget
        {
            return false;
        }
        return true;
    }

    /// Bruteforce the the value of the maximum valued computer that can be
    /// created which costs less that or equal to the supplied budget.
    fn brute_best_computer_value(&self) -> i32 {
        let mut comp_idxs = vec![0; self.components.len()];
        let mut best_value = 0;

        /* Iterate over the component combinations to find the best value one. */
        for _ in 0..self.components.iter().map(|x| x.len()).product() {
            let computer_cost = (0..self.components.len())
                .map(|x| self.components[x][comp_idxs[x]].0)
                .sum::<usize>();

            /* Has a new best value computer been found? */
            if computer_cost <= self.budget {
                let computer_value = (0..self.components.len())
                    .map(|x| self.components[x][comp_idxs[x]].1)
                    .sum::<usize>();

                if computer_value > best_value {
                    best_value = computer_value;
                }
            }

            /* Increment onto the next combination of components. */
            for ct_idx in 0..self.components.len() {
                if comp_idxs[ct_idx] < self.components[ct_idx].len() - 1 {
                    comp_idxs[ct_idx] += 1;
                    break;
                } else {
                    comp_idxs[ct_idx] = 0;
                }
            }
        }
        return best_value as i32;
    }
}
