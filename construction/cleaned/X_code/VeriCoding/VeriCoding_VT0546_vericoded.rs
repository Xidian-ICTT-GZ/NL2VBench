use vstd::prelude::*;
verus! {
struct SFC64State {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}
fn sfc64(seed: Option<u64>) -> (state: SFC64State){
    match seed {
        None => {
            SFC64State { a: 0u64, b: 0u64, c: 0u64, counter: 0u64 }
        }
        Some(x) => {
            let a = if x == 0u64 { 1u64 } else { x };
            SFC64State { a, b: 0u64, c: 0u64, counter: 0u64 }
        }
    }
}
}
fn main() {}