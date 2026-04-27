use vstd::prelude::*;
verus! {
fn has_common_element(list1: &Vec<i32>, list2: &Vec<i32>) -> (result: bool){
    let mut i = 0;
    while i < list1.len()
    {
        let mut j = 0;
        while j < list2.len()
        {
            if list1[i] == list2[j] {
                return true;
            }
            j += 1;
        }
        i += 1;
    }
    false
}
fn main() {}
} 