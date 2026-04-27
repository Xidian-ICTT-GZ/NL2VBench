use vstd::prelude::*;
verus! {
struct Laguerre {
    coef: Vec<f32>,
    domain: [f32; 2],
    window: [f32; 2],
}
fn make_unit_interval() -> (result: [f32; 2]){
    [0.0f32, 1.0f32]
}
fn assemble_laguerre(coef: Vec<f32>, domain: [f32; 2], window: [f32; 2]) -> (result: Laguerre){
    Laguerre { coef, domain, window }
}
fn make_laguerre(coefficients: Vec<f32>) -> (result: Laguerre){
    let d = make_unit_interval();
    let w = make_unit_interval();
    let r = assemble_laguerre(coefficients, d, w);
    r
}
}
fn main() {}