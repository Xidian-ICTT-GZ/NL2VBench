use vstd::prelude::*;
verus! {
fn insert_delimiter(numbers: Vec<i8>, delimiter: i8) -> (result: Vec<i8>){
    let n_len = numbers.len();
    if n_len <= 1 {
        return numbers;
    }
    let mut result: Vec<i8> = Vec::new();
    result.push(numbers[0]);
    let mut i: usize = 1;
    while i < n_len
    {
        result.push(delimiter);
        result.push(numbers[i]);
        i = i + 1;
    }
    result
}
}
fn main() {}