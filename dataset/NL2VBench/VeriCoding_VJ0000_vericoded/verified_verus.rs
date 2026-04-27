use vstd::prelude::*;

verus!{

fn min(a: int, b: int) -> (result: int)
   ensures
        result == a || result == b,
        result <= a && result <= b,
{ 
    if a < b { a } else { b } 
}

}
fn main() {}