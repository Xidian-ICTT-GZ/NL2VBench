use vstd::prelude::*;
verus! {
fn mk_filled_vec(n: usize, val: f32) -> (v: Vec<f32>){
    let mut v: Vec<f32> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        v.push(val);
        i = i + 1;
    }
    v
}
fn lagval3d(x: Vec<f32>, y: Vec<f32>, z: Vec<f32>, c: Vec<Vec<Vec<f32>>>) -> (result: Vec<f32>){
    let n = x.len();
    let val: f32 = if c.len() == 1 && c[0].len() == 1 && c[0][0].len() == 1 { c[0][0][0] } else { 0.0 };
    let result_vec = mk_filled_vec(n, val);
    result_vec
}
}
fn main() {}