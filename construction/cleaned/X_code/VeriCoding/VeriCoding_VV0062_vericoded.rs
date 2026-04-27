use vstd::prelude::*;
verus! {
fn max_usize(a: usize, b: usize) -> usize { if a >= b { a } else { b } }
fn remove_duplicates(nums: &Vec<i32>) -> (result: usize){
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    let mut count: usize = 1;
    let mut i: usize = 1;
    while i < len
    {
        if nums[i] != nums[i - 1] {
            count = count + 1;
        }
        i = i + 1;
    }
    count
}
}
fn main() {}