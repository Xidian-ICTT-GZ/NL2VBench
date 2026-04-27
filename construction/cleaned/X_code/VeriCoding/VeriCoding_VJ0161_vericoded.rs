use vstd::prelude::*;
verus! {
fn rolling_max(numbers: Vec<i32>) -> (result: Vec<i32>){
    let mut result = Vec::new();
    let mut max_so_far = i32::MIN;
    for i in 0..numbers.len()
    {
        if i == 0 {
            max_so_far = numbers[i];
        } else {
            if numbers[i] > max_so_far {
                max_so_far = numbers[i];
            }
        }
        result.push(max_so_far);
    }
    result
}
}
fn main() {}