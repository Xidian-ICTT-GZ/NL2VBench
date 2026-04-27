use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn MaxDifference(a: Vec<int>) -> (diff: int){
    let mut minVal = a[0];
    let mut maxVal = a[0];
    let mut i = 1;
    while i < a.len() 
    {
        if a[i] < minVal {
            minVal = a[i];
        } else if a[i] > maxVal {
            maxVal = a[i];
        }
        i += 1;
    }
    maxVal - minVal
}
fn main() {}
}