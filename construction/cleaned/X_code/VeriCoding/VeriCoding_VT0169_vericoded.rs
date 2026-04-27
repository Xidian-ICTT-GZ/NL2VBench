use vstd::prelude::*;
verus! {
fn choose(indices: Vec<u8>, choices: Vec<Vec<f64>>) -> (result: Vec<f64>){
    let mut result: Vec<f64> = Vec::new();
    let mut i: usize = 0;
    while i < indices.len()
    {
        let index = indices[i];
        let value = choices[index as usize][i];
        result.push(value);
        i = i + 1;
    }
    result
}
}
fn main() {}