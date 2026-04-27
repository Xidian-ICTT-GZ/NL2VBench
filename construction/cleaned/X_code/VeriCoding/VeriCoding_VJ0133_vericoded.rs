use vstd::prelude::*;
verus!{
fn get_element_check_property(arr: Vec<u64>, i: usize) -> (ret: u64){
    let ret_val = arr[i];
    ret_val
}
}
fn main() {}