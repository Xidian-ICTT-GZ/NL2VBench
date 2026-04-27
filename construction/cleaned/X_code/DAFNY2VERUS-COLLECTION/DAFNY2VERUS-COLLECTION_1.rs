use vstd::prelude::*;
verus! {
fn max(a: &Vec<i32>) -> (x: usize){
    let mut x: usize = 0;
    let mut y: usize = a.len() - 1;
    let mut m: usize = y; 
    while x != y
    {
        if a[x] <= a[y] {
            x = x + 1;
            m = y;
        } else {
            y = y - 1;
            m = x;
        }
    }
    x
}
fn main() {}
}