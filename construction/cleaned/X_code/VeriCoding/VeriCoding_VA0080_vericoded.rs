use vstd::prelude::*;
verus! {
fn solve(r: i8) -> (result: Vec<i8>){
    if r > 4 && r % 2 == 1 {
        let y: i8 = (r - 3) / 2;
        let mut v: Vec<i8> = Vec::new();
        v.push(1i8);
        v.push(y);
        v
    } else {
        Vec::new()
    }
}
}
fn main() {}