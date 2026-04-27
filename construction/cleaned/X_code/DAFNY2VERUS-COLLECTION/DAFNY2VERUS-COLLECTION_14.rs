use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn max(a: &Vec<i32>) -> (max: i32){
    if a.len() == 0 {
        return 0;
    }
    let mut max = a[0];
    let mut i = 1;
    while i < a.len()
    {
        if a[i] > max {
            max = a[i];
        }
        i = i + 1;
    }
    max
}
} 
fn main() {}