use vstd::prelude::*;
verus! {
fn flipud(m: Vec<f32>) -> (result: Vec<f32>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < m.len()
    {
        result.push(m[m.len() - 1 - i]);
        i = i + 1;
    }
    result
}
}
fn main() {}