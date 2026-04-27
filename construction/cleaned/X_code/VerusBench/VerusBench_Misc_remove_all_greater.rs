use vstd::prelude::*;
fn main() {}
verus!{
pub fn remove_all_greater(v: Vec<i32>, e: i32) -> (result: Vec<i32>){  
    let mut i: usize = 0;
    let vlen = v.len();
    let mut result: Vec<i32> = vec![];
    while (i < v.len()) 
    {  
        if (v[i] <= e) { 
            result.push(v[i]); 
        }
        i = i + 1;
    }  
    result
}
}