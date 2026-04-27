use vstd::prelude::*;
fn main() {}

verus! {

fn rotate_right(list: &Vec<u32>, n: usize) -> (new_list: Vec<u32>){
    let rotation = n % list.len();
    let split_index = list.len() - rotation;
    let mut new_list = Vec::with_capacity(list.len());
    let mut index = split_index;

    while index < list.len() {
        new_list.push(list[index]);
        index += 1;
    }
    index = 0;
    while index < split_index {
        new_list.push(list[index]);
        index += 1;
    }
    new_list
}

} // verus!