use vstd::prelude::*;
fn main() {}
verus!{
pub fn simple_nested(a: &mut Vec<i32>, b: &Vec<i32>, N: i32) -> (sum: i32){  
    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while (i < N as usize) 
    {  
        a.set(i, b[i] + 1);
        let mut j: usize = 0;
        while (j < i)
        {
            a.set(i, a[i] - 1);
            j = j + 1;
        }
        sum = sum + a[i];
        i = i + 1;
    }  
    sum
}
}