use vstd::prelude::*;
verus! {
fn split_array(list: &Vec<i32>, l: usize) -> (new_list: (Vec<i32>, Vec<i32>)){
    let mut first_part = Vec::new();
    let mut second_part = Vec::new();
    let mut i = 0;
    while i < list.len()
    {
        if i < l {
            first_part.push(list[i]);
        } else {
            second_part.push(list[i]);
        }
        i += 1;
    }
    (first_part, second_part)
}
}
fn main() {}