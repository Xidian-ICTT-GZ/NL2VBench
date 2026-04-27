use vstd::prelude::*;
verus! {
fn min2(a: i8, b: i8) -> i8 {
    if a < b { a } else { b }
}
fn max2(a: i8, b: i8) -> i8 {
    if a > b { a } else { b }
}
fn largest_smallest_integers(arr: Vec<i8>) -> (result: (Option<i8>, Option<i8>))
{
    if arr.len() == 0 {
        return (Option::None, Option::None);
    }
    let mut i: usize = 0;
    let mut min_v: i8 = 0;
    let mut max_v: i8 = 0;
    let mut initialized: bool = false;
    while i < arr.len()
    {
        let v = arr[i];
        if !initialized {
            min_v = v;
            max_v = v;
            initialized = true;
        } else {
            min_v = min2(min_v, v);
            max_v = max2(max_v, v);
        }
        i += 1;
    }
    if initialized {
        (Option::Some(max_v), Option::Some(min_v))
    } else {
        (Option::None, Option::None)
    }
}
}
fn main() {}