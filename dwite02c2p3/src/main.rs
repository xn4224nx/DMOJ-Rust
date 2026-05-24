/*
 * DWITE '02 R2 #3 - The Game of Life
 * https://dmoj.ca/problem/dwite02c2p3
 */

const ALIVE_SQR: char = 'X';
const DEAD_SQR: char = '.';

struct GameOfLife {
    world: Vec<Vec<bool>>,
}

impl GameOfLife {
    fn new() -> Self {
        let mut buffer = String::new();

        /* Read the world dimensions */
        std::io::stdin().read_line(&mut buffer).unwrap();
        let dims = buffer
            .trim_end()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut world = vec![vec![false; dims[1]]; dims[0]];

        /* Read the worlds initial state. */
        for row_idx in 0..dims[0] {
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();

            for (col_idx, cell) in buffer.chars().enumerate() {
                if cell == ALIVE_SQR {
                    world[row_idx][col_idx] = true;
                }
            }
        }
        return GameOfLife { world };
    }

    /// How many cells are currently alive in the game?
    fn count_live_cells(&self) -> usize {
        return (0..self.world.len())
            .map(|x| self.world[x].iter().filter(|x| **x).count())
            .sum();
    }

    /// Generate the next iteration of the world.
    fn next_generation(&mut self) {
        let mut nxt_world = vec![vec![false; self.world[0].len()]; self.world.len()];

        for row_idx in 0..self.world.len() {
            for col_idx in 0..self.world[0].len() {
                let mut curr_neigh = 0;

                /* The cell straight above. */
                if row_idx > 0 && self.world[row_idx - 1][col_idx] {
                    curr_neigh += 1;
                }

                /* The cell straight below. */
                if row_idx < self.world.len() - 1 && self.world[row_idx + 1][col_idx] {
                    curr_neigh += 1;
                }

                /* The cell to the left. */
                if col_idx > 0 && self.world[row_idx][col_idx - 1] {
                    curr_neigh += 1;
                }

                /* The cell to the right. */
                if col_idx < self.world[row_idx].len() - 1 && self.world[row_idx][col_idx + 1] {
                    curr_neigh += 1;
                }

                /* The cell in the top left. */
                if col_idx > 0 && row_idx > 0 && self.world[row_idx - 1][col_idx - 1] {
                    curr_neigh += 1;
                }

                /* The cell in the top right. */
                if col_idx < self.world[row_idx].len() - 1
                    && row_idx > 0
                    && self.world[row_idx - 1][col_idx + 1]
                {
                    curr_neigh += 1;
                }

                /* The cell in the bottom left. */
                if col_idx > 0
                    && row_idx < self.world.len() - 1
                    && self.world[row_idx + 1][col_idx - 1]
                {
                    curr_neigh += 1;
                }

                /* The cell in the bottom right. */
                if col_idx < self.world[row_idx].len() - 1
                    && row_idx < self.world.len() - 1
                    && self.world[row_idx + 1][col_idx + 1]
                {
                    curr_neigh += 1;
                }

                /* Change the cell based on the number of neighbours */
                nxt_world[row_idx][col_idx] =
                    curr_neigh == 3 || (self.world[row_idx][col_idx] && curr_neigh == 2);
            }
        }
        self.world = nxt_world;
    }

    /// After a set number of generations show the number of alive cells
    fn cell_after_generations(&mut self, gens: &Vec<usize>) {
        let mut print_gens_ordered = gens.clone();
        print_gens_ordered.sort();
        let mut curr_prnt_gen_idx = 0;

        assert!(!print_gens_ordered.is_empty());

        /* Iterate the world and print the number of alive cells at the intervals. */
        for gen_idx in 0..=print_gens_ordered[print_gens_ordered.len() - 1] {
            if gen_idx == print_gens_ordered[curr_prnt_gen_idx] {
                curr_prnt_gen_idx += 1;
                println!("{}", self.count_live_cells());
            }
            self.next_generation();
        }
    }

    /// Show what the current state of the world is
    fn state(&self) -> String {
        let mut view = String::new();

        for row_idx in 0..self.world.len() {
            for col_idx in 0..self.world[0].len() {
                view.push(if self.world[row_idx][col_idx] {
                    ALIVE_SQR
                } else {
                    DEAD_SQR
                });
            }
            view.push('\n');
        }
        return view;
    }
}

fn main() {
    GameOfLife::new().cell_after_generations(&vec![1, 5, 10, 50, 100]);
}
