use vstd::prelude::*;
verus! {
fn smallest_num(nums: &Vec<i32>) -> (min: i32){
    let mut min = nums[0];
    let mut index = 1;
    while index < nums.len()
    {
        if nums[index] < min {
            min = nums[index];
        }
        index += 1;
    }
    min
}
fn main() {}
} 