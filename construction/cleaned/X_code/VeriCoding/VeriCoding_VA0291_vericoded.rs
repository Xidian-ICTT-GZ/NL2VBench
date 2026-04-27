use vstd::prelude::*;
verus! {
fn eligible_u8(p: u8, k: u8) -> bool
{
    if p <= 5u8 {
        let rem: u8 = 5u8 - p;
        rem >= k
    } else {
        false
    }
}
fn solve(n: u8, k: u8, participations: Vec<u8>) -> (result: u8)
{
    let len: usize = participations.len();
    let n_usize: usize = n as usize;
    let limit: usize = if len < n_usize { len } else { n_usize };
    let mut i: usize = 0;
    let mut eligible_count: u8 = 0;
    while i < limit
    {
        let p: u8 = participations[i];
        if eligible_u8(p, k) {
            if eligible_count < 255u8 {
                eligible_count = eligible_count + 1;
            }
        }
        i = i + 1;
    }
    let result: u8 = eligible_count / 3u8;
    result
}
}
fn main() {}