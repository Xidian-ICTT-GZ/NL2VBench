use vstd::prelude::*;
verus! {

fn find_max(nums: Vec<i32>) -> (ret:i32){
    let mut max_val = nums[0];
    let mut i = 1;
    while i < nums.len()
    {
        if nums[i] > max_val {
            max_val = nums[i];
        }
        i += 1;
    }
    max_val
}
}
fn main() {}