use vstd::prelude::*;

fn main() {}

verus! {

fn count_true(arr: &Vec<bool>) -> (count: u64){
    let mut index = 0;
    let mut counter = 0;

    while index < arr.len() {
        if (arr[index]) {
            counter += 1;
        }
        index += 1;
    }
    counter
}

} // verus!