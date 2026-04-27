use vstd::prelude::*;
verus! {
fn vec2(a: f32, b: f32) -> (result: Vec<f32>){
    let mut v: Vec<f32> = Vec::new();
    v.push(a);
    v.push(b);
    v
}
fn legline(off: f32, scl: f32) -> (result: Vec<f32>){
    let v = vec2(off, scl);
    v
}
}
fn main() {}