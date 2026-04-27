use vstd::prelude::*;
verus! {
fn prime_num(n: u64) -> (result: bool){
    let mut i: u64 = 2;
    while i < n
    {
        if n % i == 0 {
            return false;
        }
        i = i + 1;
    }
    true
}
}
fn main() {}