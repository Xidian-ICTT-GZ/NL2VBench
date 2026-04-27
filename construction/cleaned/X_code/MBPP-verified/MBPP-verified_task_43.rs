use vstd::prelude::*;
verus! {
fn sum(arr: &Vec<i64>) -> (sum: i128){
    let mut index = 0;
    let mut sum = 0i128;
    while index < arr.len()
    {
        sum = sum + arr[index] as i128;
        index += 1;
    }
    sum
}
fn main() {}
} 