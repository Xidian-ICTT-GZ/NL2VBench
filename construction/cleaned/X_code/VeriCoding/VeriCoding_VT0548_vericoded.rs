use vstd::prelude::*;
verus! {
struct BitGenerator {
    state: u64,
    seed: Option<u64>,
}
struct Generator {
    bit_generator: BitGenerator,
    initialized: bool,
}
fn compute_state(seed: Option<u64>) -> (s: u64){
    let s = match seed {
        Some(x) => if x == 0u64 { 1u64 } else { x },
        None => 0u64,
    };
    s
}
fn default_rng(seed: Option<u64>) -> (result: Generator){
    let s = compute_state(seed);
    let bg = BitGenerator { state: s, seed };
    let g = Generator { bit_generator: bg, initialized: true };
    g
}
}
fn main() {}