use vstd::prelude::*;
verus! {
fn smallest_list_length(list: &Vec<Vec<i32>>) -> (min: usize){
    let mut min = list[0].len();
    let mut index = 1;
    while index < list.len()
    {
        if (&list[index]).len() < min {
            min = (&list[index]).len();
        }
        index += 1;
    }
    min
}
fn main() {}
} 