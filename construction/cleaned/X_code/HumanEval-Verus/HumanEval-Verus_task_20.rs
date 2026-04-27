use vstd::prelude::*;
verus! {
fn rolling_max(numbers: Vec<i32>) -> (result: Vec<i32>){
    let mut max_so_far = i32::MIN;
    let mut result = Vec::with_capacity(numbers.len());
    for pos in 0..numbers.len()
    {
        let number = numbers[pos];
        if number > max_so_far {
            max_so_far = number;
        }
        result.push(max_so_far);
    }
    result
}
} 