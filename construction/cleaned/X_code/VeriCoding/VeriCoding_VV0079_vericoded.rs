use vstd::prelude::*;
verus! {
fn two_sum(nums: &Vec<i32>, target: i32) -> (result: Vec<usize>){
    let mut i: usize = 0;
    while i < nums.len()
    {
        let mut j: usize = 0;
        while j < i
        {
            let sum = nums[i] as i64 + nums[j] as i64;
            if sum == target as i64 {
                let mut result = Vec::new();
                result.push(j);
                result.push(i);
                return result;
            }
            j = j + 1;
        }
        i = i + 1;
    }
    unreached()
}
}
fn main() {}