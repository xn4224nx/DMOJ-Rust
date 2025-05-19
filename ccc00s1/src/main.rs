/*
 * CCC '00 S1 - Slot Machines
 * https://dmoj.ca/problem/ccc00s1
 */

fn main() {
    let slot_payouts = vec![30, 60, 9];
    let slot_plays = vec![35, 100, 10];
    let mut slot_idx = 0;
    let mut plays = 0;

    /* Read the intial number of quarters. */
    let mut quarters = read_num();

    /* Read the intial states of the slot machines. */
    let mut slots: Vec<u32> = (0..3).map(|_| read_num()).collect();

    assert_eq!(slots.len(), slot_payouts.len());
    assert_eq!(slots.len(), slot_plays.len());

    /* Determine the number of plays it takes to go bankrupt. */
    while quarters > 0 {
        plays += 1;
        quarters -= 1;

        /* Does the slot machine pay out when played? */
        slots[slot_idx] += 1;
        if slots[slot_idx] == slot_plays[slot_idx] {
            quarters += slot_payouts[slot_idx];
            slots[slot_idx] = 0;
        }

        /* Move onto the next slot machine. */
        slot_idx = (slot_idx + 1) % slots.len();
    }
    println!("Martha plays {} times before going broke.", plays);
}

fn read_num() -> u32 {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().parse::<u32>().unwrap();
}
