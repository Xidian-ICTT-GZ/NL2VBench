use vstd::prelude::*;
verus! {
fn append_vectors<T: Copy>(v1: &Vec<T>, v2: &Vec<T>) -> (result: Vec<T>){
    let mut result = Vec::new();
    let mut i = 0;
    while i < v1.len()
    {
        result.push(v1[i]);
        i += 1;
    }
    let mut j = 0;
    while j < v2.len()
    {
        result.push(v2[j]);
        j += 1;
    }
    result
}
fn replace_last_element(first: &Vec<i32>, second: &Vec<i32>) -> (result: Vec<i32>){
    let mut prefix_vec = Vec::new();
    let mut i = 0;
    while i < first.len() - 1
    {
        prefix_vec.push(first[i]);
        i += 1;
    }
    let result = append_vectors(&prefix_vec, second);
    result
}
fn main() {
}
}