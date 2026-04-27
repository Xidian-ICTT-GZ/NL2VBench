use vstd::prelude::*;
verus! {
fn square_nums(nums: &Vec<i32>) -> (squared: Vec<i32>){
    let mut squared = Vec::new();
    let mut i = 0;
    while i < nums.len()
    {
        let val = nums[i];
        squared.push(val * val);
        i += 1;
    }
    squared
}
}
fn main() {}