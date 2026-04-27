use vstd::prelude::*;
verus! {
fn binary_search_recursive(v: &[i32], elem: i32, c: isize, f: isize) -> (p: isize){
    if c > f {
        f
    } else {
        let m = c + (f - c) / 2;
        if v[m as usize] <= elem {
            binary_search_recursive(v, elem, m + 1, f)
        } else {
            binary_search_recursive(v, elem, c, m - 1)
        }
    }
}
}
fn main() {}