use vstd::prelude::*;
verus! {
fn solve(n: i8, m: i8) -> (result: bool){
    let min_val = if n < m { n } else { m };
    (min_val % 2) == 1
}
}
fn main() {}