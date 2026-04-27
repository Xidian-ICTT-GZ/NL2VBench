use vstd::prelude::*;
verus! {
fn all_sequence_equal_length(seq: &Vec<Vec<i32>>) -> (result: bool){
    let first_len = seq[0].len();
    let mut i = 1;
    while i < seq.len()
    {
        if seq[i].len() != first_len {
            return false;
        }
        i += 1;
    }
    true
}
}
fn main() {}