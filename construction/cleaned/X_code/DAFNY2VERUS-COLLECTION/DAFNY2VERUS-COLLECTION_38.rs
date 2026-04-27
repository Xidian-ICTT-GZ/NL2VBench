use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn max_array(a: Vec<int>) -> (m: int){
    let mut m = a[0];
    let mut index = 1;
    while index < a.len()
    {
        m = if m > a[index] { m } else { a[index] };
        index = index + 1;
    }
    m
}
fn main() {}
}