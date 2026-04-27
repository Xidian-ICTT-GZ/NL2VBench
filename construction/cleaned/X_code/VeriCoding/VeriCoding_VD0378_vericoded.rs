use vstd::prelude::*;
verus! {
fn removeElement(nums: &mut Vec<i32>, val: i32) -> (i: usize){
    let mut i: usize = 0;
    let mut j: usize = 0;
    while j < nums.len()
    {
        if nums[j] != val {
            nums.set(i, nums[j]);
            i = i + 1;
        }
        j = j + 1;
    }
    i
}
}
fn main() {}