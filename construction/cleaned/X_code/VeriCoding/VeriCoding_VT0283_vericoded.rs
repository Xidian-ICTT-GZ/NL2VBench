use vstd::prelude::*;
verus! {
fn acos_map(xi: i8) -> (r: i8){
    if xi == -1 { 3 } else if xi == 1 { 0 } else { 1 }
}
fn arccos(x: Vec<i8>) -> (result: Vec<i8>){
    let n = x.len();
    let mut result: Vec<i8> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        let xi = x[i];
        let ri = acos_map(xi);
        let cur = i;
        result.push(ri);
        i = cur + 1;
    }
    result
}
}
fn main() {}