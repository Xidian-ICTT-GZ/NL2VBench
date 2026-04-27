use vstd::prelude::*;
verus! {
fn is_greater(arr: &Vec<i32>, number: i32) -> (result: bool){
    let mut i: usize = 0;
    let mut ok: bool = true;
    while i < arr.len()
    {
        if number > arr[i] {
            i = i + 1;
        } else {
            ok = false;
            i = arr.len();
        }
    }
    ok
}
}
fn main() {}