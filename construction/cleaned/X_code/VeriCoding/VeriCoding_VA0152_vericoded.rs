use vstd::prelude::*;
verus! {
fn min_steps(x: i8) -> (result: i8){
    let mut steps: i16 = 0;
    let mut total: i16 = 0;
    while total < x as i16
    {
        total = total + 5;
        steps = steps + 1;
    }
    let result: i8 = steps as i8;
    result
}
}
fn main() {}