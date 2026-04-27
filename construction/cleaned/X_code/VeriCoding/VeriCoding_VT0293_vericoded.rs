use vstd::prelude::*;
verus! {
fn clip_elem(x: i8, min_val: i8, max_val: i8) -> (res: i8){
    if min_val <= max_val {
        if x < min_val {
            return min_val;
        } else if x > max_val {
            return max_val;
        } else {
            return x;
        }
    } else {
        return max_val;
    }
}
fn clip(arr: &Vec<i8>, min_val: i8, max_val: i8) -> (result: Vec<i8>){
    let mut result = Vec::<i8>::new();
    let mut i: usize = 0;
    while i < arr.len()
    {
        let x: i8 = arr[i];
        let c: i8 = clip_elem(x, min_val, max_val);
        result.push(c);
        i += 1;
    }
    result
}
}
fn main() {}