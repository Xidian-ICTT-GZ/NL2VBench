use vstd::prelude::*;
verus! {
fn concatenate(a: Vec<f32>, b: Vec<f32>) -> (result: Vec<f32>){
    let mut result = Vec::<f32>::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let ai = a[i];
        result.push(ai);
        i = i + 1;
    }
    let mut j: usize = 0;
    while j < b.len()
    {
        let bj = b[j];
        result.push(bj);
        j = j + 1;
    }
    result
}
}
fn main() {}