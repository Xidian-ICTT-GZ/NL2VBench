use vstd::prelude::*;

fn main() {}

verus! {

fn two_sum(nums: &Vec<u32>, target: u32) -> (r: (usize, usize)){
    let n = nums.len();
    let mut i = 0;
    let mut j = 1;

    while i < n - 1 {
        j = i + 1;
        while j < n {
            if nums[i] + nums[j] == target {
                return (i, j);
            }
            j += 1;
        }
        i += 1;
    }
    (i, j)
}

} // verus!