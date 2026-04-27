use vstd::prelude::*;
verus! {
fn linear_search(nums: Vec<i32>, target: i32) -> (ret: i32){
    let mut index: i32 = 0;
    while index < nums.len() as i32
    {
        if nums[index as usize] == target {
            return index;
        }
        index = index + 1;
    }
    -1
}
}
fn main() {}