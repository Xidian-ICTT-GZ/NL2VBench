use vstd::prelude::*;
verus! {
fn replace(v: &mut Vec<i32>, x: i32, y: i32){
    let n = v.len();
    let mut i: usize = 0;
    while i < n
    {
        let cur = v[i];
        if cur == x {
            v.set(i, y);
        } else {
        }
        i = i + 1;
    }
}
fn main() {
}
}