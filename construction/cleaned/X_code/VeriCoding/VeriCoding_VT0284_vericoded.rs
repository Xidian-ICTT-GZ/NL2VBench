use vstd::prelude::*;
verus! {
fn arccosh_elem(x: f32) -> f32 {
    x
}
fn arccosh(x: Vec<f32>) -> (result: Vec<f32>){
    let n = x.len();
    let mut r: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let xi = x[i];
        r.push(arccosh_elem(xi));
        i = i + 1;
    }
    r
}
}
fn main() {}