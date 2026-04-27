use vstd::prelude::*;
verus! {
fn polymulx(c: Vec<f32>) -> (result: Vec<f32>){
    let mut result: Vec<f32> = Vec::new();
    result.push(0.0f32);
    let mut j: usize = 0usize;
    while j < c.len()
    {
        let v = c[j];
        result.push(v);
        j = j + 1usize;
    }
    result
}
}
fn main() {}