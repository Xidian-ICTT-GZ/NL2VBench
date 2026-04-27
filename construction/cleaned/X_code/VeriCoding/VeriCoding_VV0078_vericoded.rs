use vstd::prelude::*;
verus! {
fn two_sum(nums: Vec<i32>, target: i32) -> (result: Option<(usize, usize)>){
    let mut i: usize = 0;
    while i < nums.len()
    {
        let mut j: usize = i + 1;
        while j < nums.len()
        {
            if nums[i].checked_add(nums[j]).is_some() && nums[i] + nums[j] == target {
                return Some((i, j));
            }
            j = j + 1;
        }
        i = i + 1;
    }
    None
}
}
fn main() {}