use vstd::prelude::*;
verus! {
fn contains(list: &Vec<i32>, element: i32) -> (result: bool){
    let mut i = 0;
    while i < list.len()
    {
        if list[i] == element {
            return true;
        }
        i = i + 1;
    }
    false
}
fn has_common_element(list1: &Vec<i32>, list2: &Vec<i32>) -> (result: bool){
    let mut i = 0;
    while i < list1.len()
    {
        let elem = list1[i];
        if contains(list2, elem) {
            return true;
        }
        i = i + 1;
    }
    false
}
}
fn main() {}