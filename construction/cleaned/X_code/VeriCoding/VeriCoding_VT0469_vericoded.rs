use vstd::prelude::*;
verus! {
fn make_pair(a: f32, b: f32) -> (result: Vec<f32>){
    let mut v = Vec::<f32>::new();
    v.push(a);
    v.push(b);
    v
}
fn lagline(off: f32, scl: f32) -> (result: Vec<f32>){
    let v = make_pair(off, scl);
    v
}
}
fn main() {}