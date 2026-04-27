use vstd::prelude::*;
use vstd::seq::*;
use vstd::map::*;
use vstd::set::*;
verus! {
fn min_array(a: Vec<int>) -> (r: int){
    let mut r = a[0];
    let mut i = 1;
    while i < a.len() 
    {
        if r > a[i] {
            r = a[i];
        }
        i = i + 1;
    }
    r
}
fn main() {}
}