use vstd::prelude::*;

fn main() {}
verus! {

fn test_prime(candidate: u64) -> (result: bool){
    let mut factor: u64 = 2;
    while factor < candidate {
        if candidate % factor == 0 {
            return false;
        }
        factor = factor + 1;
    }
    true
}

} // verus!