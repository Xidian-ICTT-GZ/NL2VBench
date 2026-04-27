use vstd::prelude::*;

fn main() {}

verus! {

fn min_sublist(seq: &Vec<Vec<i32>>) -> (min_list: &Vec<i32>){
    let mut min_list = &seq[0];
    let mut index = 1;

    while index < seq.len() {
        if ((seq[index]).len() < min_list.len()) {
            min_list = &seq[index];
        }
        index += 1;
    }
    min_list
}

} // verus!