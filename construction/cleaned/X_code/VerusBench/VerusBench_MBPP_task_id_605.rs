use vstd::prelude::*;

fn main() {}

verus! {

fn prime_num(n: u64) -> (result: bool){
    if n <= 1 {
        return false;
    }
    let mut index = 2;
    while index < n {
        if ((n % index) == 0) {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!