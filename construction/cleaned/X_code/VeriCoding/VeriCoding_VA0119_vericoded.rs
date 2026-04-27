use vstd::prelude::*;
verus! {
fn solve(n: i8, m: i8) -> (result: (i8, i8)){
    let vasya: i8;
    let petya: i8;
    if n < m {
        petya = m - 1;
        vasya = n;
    } else {
        petya = n - 1;
        vasya = m;
    }
    (petya, vasya)
}
}
fn main() {}