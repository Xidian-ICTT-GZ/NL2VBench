use vstd::prelude::*;
verus! {
fn min_inspectors(n: i8, d: i8) -> (result: i8){
    let coverage_diameter = 2 * d + 1;
    let result = ((n - 1) / coverage_diameter) + 1;
    result
}
}
fn main() {}