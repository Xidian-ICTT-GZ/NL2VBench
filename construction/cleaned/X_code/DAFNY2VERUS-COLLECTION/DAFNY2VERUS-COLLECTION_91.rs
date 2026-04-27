use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn find_max(a: &Vec<i32>) -> (max: i32){
    let mut max = a[0];
    let mut i = 1;
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
    let a1 = vec![1, 2, 3];
    let m1 = find_max(&a1);
    let a2 = vec![3, 2, 1];
    let m2 = find_max(&a2);
    let a3 = vec![2, 3, 1];
    let m3 = find_max(&a3);
    let a4 = vec![1, 2, 2];
    let m4 = find_max(&a4);
    let a5 = vec![1];
    let m5 = find_max(&a5);
    let a6 = vec![1, 1, 1];
    let m6 = find_max(&a6);
    println!("All tests passed!");
}