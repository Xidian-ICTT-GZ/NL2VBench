use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: Vec<i8>){
    let r: i8 = n % 3;
    if r == 0_i8 {
        let a: i8 = n / 3;
        let mut v = Vec::new();
        v.push(a);
        v.push(0_i8);
        v.push(0_i8);
        v
    } else if r == 1_i8 {
        if n < 7_i8 {
            let mut v = Vec::new();
            v.push(-1_i8);
            v
        } else {
            let a: i8 = (n - 7_i8) / 3;
            let mut v = Vec::new();
            v.push(a);
            v.push(0_i8);
            v.push(1_i8);
            v
        }
    } else {
        if n < 5_i8 {
            let mut v = Vec::new();
            v.push(-1_i8);
            v
        } else {
            let a: i8 = (n - 5_i8) / 3;
            let mut v = Vec::new();
            v.push(a);
            v.push(1_i8);
            v.push(0_i8);
            v
        }
    }
}
}
fn main() {}