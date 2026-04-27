use vstd::prelude::*;
verus! {
fn size(a: &Vec<f64>) -> (result: usize){
    let r = a.len();
    r
}
}
fn main() {}