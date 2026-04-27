use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: (i8, char)){
    let r: i8 = n % 4i8;
    let a: i8;
    let b: char;
    if r == 1 {
        a = 0;
        b = 'A';
    } else if r == 2 {
        a = 1;
        b = 'B';
    } else if r == 3 {
        a = 2;
        b = 'A';
    } else {
        a = 1;
        b = 'A';
    }
    (a, b)
}
}
fn main() {}