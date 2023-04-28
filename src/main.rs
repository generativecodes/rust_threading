use rand::Rng;
use rayon::prelude::*;

fn main() {
    let rand_sequence: Vec<u64> = (0..1_000_000_000).into_par_iter().map(|x| x).collect();

    let sum: u64 = rand_sequence.par_iter().map(|x| *x as u64).sum();
    dbg!(sum);
}
