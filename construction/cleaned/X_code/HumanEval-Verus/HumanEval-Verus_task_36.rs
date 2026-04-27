use vstd::arithmetic::div_mod::*;
use vstd::arithmetic::mul::*;
use vstd::assert_by_contradiction;
use vstd::calc;
use vstd::prelude::*;
verus! {
pub fn factorize(n: u8) -> (factorization: Vec<u8>){
    let mut factorization = vec![];
    let mut k = n;
    let mut m = 2u16;
    while (m <= n as u16)
    {
        if (k as u16 % m == 0) {
            let l = factorization.len();
            factorization.insert(l, m as u8);
            k = k / m as u8;
        } else {
            m = m + 1;
        }
    }
    return factorization;
}
} 
fn main() {
    print!("{:?}", factorize(254));
}