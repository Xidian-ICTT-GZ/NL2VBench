use vstd::prelude::*;
verus! {
fn arctan_approximation(x: i32) -> (result: i32){
    if x == 0 {
        0
    } else if x > 10 {
        2
    } else if x < -10 {
        -2
    } else if x > 0 {
        1
    } else {
        -1
    }
}
fn arctan(x: Vec<i32>) -> (result: Vec<i32>){
    let mut result: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < x.len()
    {
        let approx = arctan_approximation(x[i]);
        result.push(approx);
        i += 1;
    }
    result
}
}
fn main() {}