use vstd::prelude::*;
verus! {
fn cube_element(nums: &Vec<i32>) -> (cubed: Vec<i32>){
    let mut cubed_array: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < nums.len()
    {
        cubed_array.push(nums[i] * nums[i] * nums[i]);
        i += 1;
    }
    cubed_array
}
fn main() {}
} 