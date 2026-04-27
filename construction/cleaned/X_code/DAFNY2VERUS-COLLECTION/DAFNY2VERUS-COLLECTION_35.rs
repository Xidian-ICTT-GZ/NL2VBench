use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn max_array(a: &Vec<i32>) -> (max: i32){
    let mut i: usize = 1;
    let mut max = a[0];
    while i < a.len()
    {
        if a[i] > max {
            max = a[i];
        }
        i += 1;
    }
    max
}
} 
fn main() {
    let arr = vec![-11, 2, 42, -4];
    let res = max_array(&arr);
    println!("All tests passed!");
}