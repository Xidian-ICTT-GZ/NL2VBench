use vstd::prelude::*;
fn main() {}

verus!{

fn tail_triangle(n: u32, idx: u32, sum: &mut u32){
    if idx < n {
        let idx = idx + 1;
        *sum = *sum + idx;
        tail_triangle(n, idx, sum);
    }
}
}