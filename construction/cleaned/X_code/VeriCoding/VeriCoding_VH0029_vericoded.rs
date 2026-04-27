use vstd::prelude::*;
verus! {
fn is_prime(n: i8) -> (result: bool){
    if n < 2 {
        return false;
    }
    let mut k: i8 = 2;
    while k < n
    {
        if n % k == 0 {
            return false;
        }
        k = k + 1;
    }
    return true;
}
}
fn main() {}