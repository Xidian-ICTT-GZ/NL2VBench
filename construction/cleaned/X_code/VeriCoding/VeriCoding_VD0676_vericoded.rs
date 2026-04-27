use vstd::prelude::*;
verus! {
fn is_in_array(a: &[i32], x: i32) -> (result: bool){
    let mut i: usize = 0;
    while i < a.len()
    {
        if a[i] == x {
            return true;
        }
        i = i + 1;
    }
    return false;
}
fn contains(v: &Vec<i32>, x: i32) -> (result: bool){
    let mut i: usize = 0;
    while i < v.len()
    {
        if v[i] == x {
            return true;
        }
        i = i + 1;
    }
    return false;
}
fn remove_elements(a: &[i32], b: &[i32]) -> (result: Vec<i32>){
    let mut result: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    while i < a.len()
    {
        let x = a[i];
        let in_b = is_in_array(b, x);
        let in_result = contains(&result, x);
        if !in_b && !in_result {
            result.push(x);
        }
        i = i + 1;
    }
    result
}
}
fn main() {}