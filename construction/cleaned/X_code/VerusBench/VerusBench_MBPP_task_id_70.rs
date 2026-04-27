use vstd::prelude::*;

fn main() {}

verus! {

fn all_sequence_equal_length(seq: &Vec<Vec<i32>>) -> (result: bool){
    let mut index = 1;
    while index < seq.len() {
        if ((&seq[index]).len() != (&seq[0]).len()) {
            return false;
        }
        index += 1;
    }
    true
}

} // verus!