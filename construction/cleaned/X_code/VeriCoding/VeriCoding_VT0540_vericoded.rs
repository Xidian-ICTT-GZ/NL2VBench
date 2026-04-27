use vstd::prelude::*;
verus! {
pub struct BitGeneratorState {
    pub seed: Option<u64>,
    pub internal_state: u64,
}
fn compute_internal_state(seed: Option<u64>) -> (s: u64){
    match seed {
        Some(_) => 1u64,
        None => 0u64,
    }
}
fn numpy_random_bit_generator(seed: Option<u64>) -> (result: BitGeneratorState){
    let s = compute_internal_state(seed);
    BitGeneratorState { seed, internal_state: s }
}
}
fn main() {}