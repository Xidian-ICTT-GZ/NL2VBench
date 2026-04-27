use vstd::prelude::*;
verus! {
fn compute_min(n: i8, k: i8) -> (min_val: i8){
    if k == 0 || k == n {
        0
    } else {
        1
    }
}
fn compute_max(n: i8, k: i8) -> (max_val: i8){
    if k == 0 || k == n {
        0
    } else {
        let condition = if k > 42 { true } else { n < k * 3 };
        if condition {
            n - k
        } else {
            k * 2
        }
    }
}
fn solve(n: i8, k: i8) -> (result: Vec<i8>){
    let min_val = compute_min(n, k);
    let max_val = compute_max(n, k);
    let mut result = Vec::new();
    result.push(min_val);
    result.push(max_val);
    result
}
}
fn main() {}