use vstd::prelude::*;
verus!{

fn min(a: int, b: int) -> (result: int)
{ 
    if a < b { a } else { b } 
}

}
fn main() {}