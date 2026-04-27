use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn absx(x: Vec<i32>) -> (y: Vec<i32>){
    let mut y: Vec<i32> = Vec::new();
    let mut j: usize = 0;
    while j < x.len()
    {
        if x[j] < 0 {
            y.push(-x[j]);
        } else {
            y.push(x[j]);
        }
        j += 1;
    }
    y
}
fn main() {}
}