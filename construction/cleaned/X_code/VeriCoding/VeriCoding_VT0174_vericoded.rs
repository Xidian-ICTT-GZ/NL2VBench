use vstd::prelude::*;
verus! {
fn extract(condition: Vec<bool>, arr: Vec<f32>) -> (result: Vec<f32>){
    let n = condition.len();
    let mut res: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let b = condition[i];
        if b {
            let v = arr[i];
            res.push(v);
        }
        i = i + 1;
    }
    res
}
}
fn main() {}