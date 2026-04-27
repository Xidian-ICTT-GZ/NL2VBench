use vstd::prelude::*;
verus! {
fn hstack(a: Vec<f32>, b: Vec<f32>) -> (result: Vec<f32>){
    let mut res: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let x = a[i];
        res.push(x);
        i = i + 1;
    }
    let mut j: usize = 0;
    while j < b.len()
    {
        let x = b[j];
        res.push(x);
        j = j + 1;
    }
    res
}
}
fn main() {}