use vstd::prelude::*;
verus! {
fn longest_increasing_streak(nums: &Vec<i32>) -> (result: usize){
    if nums.len() == 0 {
        return 0;
    }
    let mut max_streak = 1;
    let mut current_streak = 1;
    let mut i = 1;
    while i < nums.len()
    {
        if nums[i] > nums[i - 1] {
            current_streak += 1;
        } else {
            current_streak = 1;
        }
        if current_streak > max_streak {
            max_streak = current_streak;
        }
        i += 1;
    }
    max_streak
}
}
fn main() {}