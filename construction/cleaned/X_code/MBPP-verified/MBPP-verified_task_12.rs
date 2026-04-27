use vstd::prelude::*;
verus! {
fn square_nums(nums: &Vec<i32>) -> (squared: Vec<i32>){
    let mut result: Vec<i32> = Vec::new();
    let mut index = 0;
    while index < nums.len()
    {
        result.push(nums[index] * nums[index]);
        index += 1
    }
    result
}
fn main() {}
} 