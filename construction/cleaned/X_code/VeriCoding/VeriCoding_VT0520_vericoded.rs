use vstd::prelude::*;
verus! {
fn polyline(off: i8, scl: i8) -> (result: [i8; 2]){
    let arr: [i8; 2] = [off, scl];
    arr
}
}
fn main() {}