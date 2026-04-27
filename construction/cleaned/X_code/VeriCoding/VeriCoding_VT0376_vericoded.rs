use vstd::prelude::*;
verus! {
struct ChebyshevPoly {
    coef: Vec<i32>,
    domain_min: i32,
    domain_max: i32,
    window_min: i32,
    window_max: i32,
}
fn chebyshev(coef: Vec<i8>) -> (result: ChebyshevPoly){
    let mut result_coef: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < coef.len()
    {
        result_coef.push(coef[i] as i32);
        i = i + 1;
    }
    ChebyshevPoly {
        coef: result_coef,
        domain_min: -1,
        domain_max: 1,
        window_min: -1,
        window_max: 1,
    }
}
}
fn main() {}