use vstd::prelude::*;
fn main() {}

verus! {
    fn test1(nums: &mut Vec<u32>)
    {
        let n = nums.len();
        if n == 0 {
            return;
        }
        for i in 1..n
        {
            let mut j = i;
            while j != 0
            {
                if nums[j - 1] > nums[j] {
                    let temp = nums[j - 1];
                    nums.set(j - 1, nums[j]);
                    nums.set(j, temp);
                }
                j -= 1;
            }
        }
    }
}