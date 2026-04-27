use vstd::prelude::*;

verus! {

fn euler_gamma_const() -> (result: f64)
{
    0.5772156649015329
}

fn euler_gamma() -> (result: f64)
{
    let gamma = euler_gamma_const();
    gamma
}

}
fn main() {}