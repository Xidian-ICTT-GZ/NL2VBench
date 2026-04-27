use vstd::prelude::*;
verus! {
struct BroadcastObject {
    x_data: Vec<f32>,
    y_data: Vec<f32>,
    shape: (usize, usize),
}
impl BroadcastObject {
}
fn broadcast(x: Vec<f32>, y: Vec<f32>) -> (result: BroadcastObject){
    let s0 = x.len();
    let s1 = y.len();
    let result = BroadcastObject { x_data: x, y_data: y, shape: (s0, s1) };
    result
}
}
fn main() {}