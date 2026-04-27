use vstd::prelude::*;
verus! {
fn solve(n_a: i8, n_b: i8, k: i8, m: i8, a: Vec<i8>, b: Vec<i8>) -> (result: &'static str){
    let a_len: usize = a.len();
    let b_len: usize = b.len();
    let k_usize: usize = k as usize;
    let m_usize: usize = m as usize;
    let k_idx: usize = k_usize - 1usize;
    let b_idx: usize = b_len - m_usize;
    let ak_i8: i8 = a[k_idx];
    let bb_i8: i8 = b[b_idx];
    let ans = if ak_i8 < bb_i8 { "YES" } else { "NO" };
    ans
}
}
fn main() {}