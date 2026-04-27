use vstd::prelude::*;
use vstd::seq::*;
verus! {
fn invert_array(a: &mut Vec<i32>){
    let mut index = 0;
    while index < a.len() / 2
    {
        let j = a.len() - 1 - index; 

        let temp = a[index];
        a[index] = a[j];
        a[j] = temp;
        index += 1;
    }
}
fn main() {}
}