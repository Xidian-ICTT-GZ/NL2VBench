use vstd::prelude::*;
verus! {
fn hermemulx(c: Vec<f64>) -> (result: Vec<f64>){
    let mut res: Vec<f64> = Vec::new();
    res.push(0.0f64);
    let mut i: usize = 0;
    while i < c.len()
    {
        let v: f64 = c[i];
        res.push(v);
        i = i + 1;
    }
    res
}
}
fn main() {}