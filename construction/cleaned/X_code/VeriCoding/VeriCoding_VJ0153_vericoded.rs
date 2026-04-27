use vstd::prelude::*;
verus! {
fn max_array(nums: &[i32]) -> (idx: usize){
    let mut max_idx = 0;
    let mut i = 1;
    while i < nums.len()
    {
        if nums[i] > nums[max_idx] {
            max_idx = i;
        }
        i += 1;
    }
    max_idx
}
}
fn main() {}