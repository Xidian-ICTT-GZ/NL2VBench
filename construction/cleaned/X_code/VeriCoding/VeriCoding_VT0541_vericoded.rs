use vstd::prelude::*;
verus! {
fn wrapping_add_u32(a: u32, b: u32) -> u32 { a.wrapping_add(b) }
fn mt19937(seed: u32) -> (state: Vec<u32>){
    let mut state: Vec<u32> = Vec::new();
    state.push(seed);
    let mut i: u32 = 0;
    while i < 623
    {
        let prev_val = state[i as usize];
        let k = i + 1;
        let shifted = prev_val >> 30;
        let xor_result = prev_val ^ shifted;
        let mult_result = 1812433253u32.wrapping_mul(xor_result);
        let next_val = mult_result.wrapping_add(k);
        state.push(next_val);
        i = i + 1;
    }
    state
}
}
fn main() {}