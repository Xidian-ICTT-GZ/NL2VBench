use vstd::prelude::*;
verus! {
struct Legendre {
    coef: Vec<f64>,
    domain: [f64; 2],
    window: [f64; 2],
    symbol: String,
}
fn mk_legendre(
    coef: Vec<f64>,
    domain: [f64; 2],
    window: [f64; 2],
    symbol: String
) -> (result: Legendre){
    Legendre { coef, domain, window, symbol }
}
}
fn main() {}