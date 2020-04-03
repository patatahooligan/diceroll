use rand::prelude::*;

pub fn single_die(max: u32) -> u32 {
    // gen_range takes [min, max) but dice are usually notated as 1dX <-> [0, X]
    // so increment the max
    thread_rng().gen_range(0, max + 1)
}
