use vstd::prelude::*;
verus! {
fn sum_to_n(n: u32) -> (sum: Option<u32>){
    if n >= 92682 {
        return None;
    }
    let mut res: u32 = 0;
    let mut sum: u32 = 0;
    let mut i: u32 = 0;
    while i < n
    {
        i += 1;
        res = i + res;
    }
    Some(res)
}
} 