use vstd::prelude::*;
verus! {
fn intersperse(numbers: &[i32], delim: i32) -> (res: Vec<i32>){
    let n_usize: usize = numbers.len();
    let mut r: Vec<i32> = Vec::new();
    if n_usize == 0 {
        return r;
    }
    r.push(numbers[0]);
    let mut i: usize = 1;
    while i < n_usize
    {
        r.push(delim);
        r.push(numbers[i]);
        i += 1;
    }
    r
}
}
fn main() {}