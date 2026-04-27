use vstd::prelude::*;
verus! {
fn solve(a: i8) -> (count: i8){
    let mut n = a;
    let mut count: i8 = 0;
    while n > 0
    {
        if n % 8 == 1 {
            count = count + 1;
        }
        n = n / 8;
    }
    count
}
}
fn main() {}