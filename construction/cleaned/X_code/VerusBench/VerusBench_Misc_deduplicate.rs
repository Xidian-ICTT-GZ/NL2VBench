#[allow(unused_imports)]
use vstd::prelude::*;
fn main() {}

verus! {
fn remove_duplicates(nums: Vec<i32>) -> (res: Vec<i32>){
    let mut res = Vec::new();
    let mut i = 0;
    while i < nums.len()
    {
        let mut found = false;
        let mut j = 0;

        while j < res.len()
        {
            if nums[i] == res[j] {
                found = true;
                break;
            }
            j += 1;
        }

        if !found {
            res.push(nums[i]);
        }
        i += 1;
    }
    res
}
}