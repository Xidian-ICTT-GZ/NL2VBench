use vstd::prelude::*;
verus! {
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}
fn maxArrayReverse(arr: &[i32]) -> (max: i32){
    let mut max = arr[0];
    let mut i: usize = 1;
    while i < arr.len()
    {
        if arr[i] > max {
            max = arr[i];
        }
        i = i + 1;
    }
    max
}
}
fn main() {}