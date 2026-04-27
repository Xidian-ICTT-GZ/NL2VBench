use vstd::prelude::*;
verus! {
fn is_non_prime(n: u64) -> (result: bool){
    let mut i = 2u64;
    while i < n
    {
        if n % i == 0 {
            return true;
        }
        i = i + 1;
    }
    return false;
}
}
fn main() {}