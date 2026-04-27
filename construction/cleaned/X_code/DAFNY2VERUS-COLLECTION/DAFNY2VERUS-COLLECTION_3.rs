use vstd::prelude::*;
verus! {
fn count_to_and_return_n(n: u64) -> (r: u64){
    let mut i: u64 = 0;
    while i < n
    {
        i = i + 1;
    }
    i  
}
fn main() {}
}