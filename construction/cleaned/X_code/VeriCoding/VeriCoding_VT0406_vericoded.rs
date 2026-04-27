use vstd::prelude::*;
verus! {
struct HermiteEPoly {
    coef: Vec<f32>,
    domain_min: f32,
    domain_max: f32,
    window_min: f32,
    window_max: f32,
}
fn hermite_e(coef: Vec<f32>) -> (result: HermiteEPoly){
    HermiteEPoly {
        coef,
        domain_min: -1.0f32,
        domain_max: 1.0f32,
        window_min: -1.0f32,
        window_max: 1.0f32,
    }
}
}
fn main() {}