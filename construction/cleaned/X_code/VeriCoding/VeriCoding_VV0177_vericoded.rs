use vstd::prelude::*;
verus! {
fn test_array_elements(a: &Vec<i32>, j: usize) -> (result: Vec<i32>){
    let n = a.len();
    let mut r: Vec<i32> = Vec::new();
    for i in 0..n
    {
        if i == j {
            r.push(60);
        } else {
            let v = a[i];
            r.push(v);
        }
    }
    r
}
}
fn main() {}