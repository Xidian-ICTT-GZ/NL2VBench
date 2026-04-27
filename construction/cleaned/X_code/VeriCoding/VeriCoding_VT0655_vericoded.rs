use vstd::prelude::*;
verus! {
fn reduce(arr: Vec<f64>) -> (result: f64){
    let len = arr.len();
    if len == 1 {
        let r = arr[0];
        r
    } else {
        let idx = len - 1;
        let r = arr[idx];
        r
    }
}
}
fn main() {}