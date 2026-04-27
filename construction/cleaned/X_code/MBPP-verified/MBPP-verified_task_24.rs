use vstd::prelude::*;
verus! {
fn interleave(s1: &Vec<i32>, s2: &Vec<i32>, s3: &Vec<i32>) -> (res: Vec<i32>){
    let new_seq_len = s1.len() * 3;
    let mut output_seq = Vec::with_capacity(new_seq_len);
    let mut index = 0;
    while index < s1.len()
    {
        output_seq.push(s1[index]);
        output_seq.push(s2[index]);
        output_seq.push(s3[index]);
        index += 1;
    }
    output_seq
}
fn main() {}
} 