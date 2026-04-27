use vstd::prelude::*;
verus! {
fn is_non_prime(n: u64) -> (result: bool){
    if n <= 1 {
        return true;
    }
    let mut index = 2;
    while index < n
    {
        if ((n % index) == 0) {
            return true;
        }
        index += 1;
    }
    false
}
fn main() {}
} 