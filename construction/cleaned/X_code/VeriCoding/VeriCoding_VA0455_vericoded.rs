use vstd::prelude::*;
verus! {
fn solve_cookie_distribution(a: u8, b: u8, n: u8, m: u8) -> (result: bool){
    let sum_ab_u16: u16 = (a as u16) + (b as u16);
    let sum_nm_u16: u16 = (n as u16) + (m as u16);
    let runtime_min_u8: u8 = if a <= b { a } else { b };
    let result: bool = (sum_ab_u16 >= sum_nm_u16) && (m <= runtime_min_u8);
    result
}
}
fn main() {}