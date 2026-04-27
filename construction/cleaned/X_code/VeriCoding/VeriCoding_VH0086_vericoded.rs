use vstd::prelude::*;
verus! {
fn solve(n: u8) -> (r: u8){
    let mut count: u8 = 0;
    let mut m = n;
    while m != 0
    {
        if m % 2 == 1 {
            count = count + 1;
        }
        m = m / 2;
    }
    count
}
}
fn main() {}