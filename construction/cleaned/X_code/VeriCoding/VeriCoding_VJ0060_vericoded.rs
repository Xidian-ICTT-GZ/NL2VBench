use vstd::prelude::*;
verus! {
fn smallest_num(nums: &Vec<i32>) -> (min: i32){
    let mut i: usize = 1usize;
    let mut m: i32 = nums[0];
    let mut idx: usize = 0usize;
    while i < nums.len()
    {
        let v: i32 = nums[i];
        if v < m {
            m = v;
            idx = i;
        }
        i = i + 1;
    }
    m
}
}
fn main() {}