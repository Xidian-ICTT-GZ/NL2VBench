use vstd::prelude::*;
verus! {
fn main_func(n: i32, k: i32) -> (k_out: i32){
    let mut k_out = k;
    let mut j: i32 = 0;
    while j < n
    {
        k_out = k_out.wrapping_sub(1);
        j = j + 1;
    }
    k_out
}
} 
fn main() {}