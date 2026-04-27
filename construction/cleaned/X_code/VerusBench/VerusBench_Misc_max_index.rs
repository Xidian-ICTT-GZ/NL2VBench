use vstd::prelude::*;
fn main() {}
verus! {

pub fn myfun1(x: &Vec<i32>) -> (max_index: usize){
    let mut max_index = 0;
    let mut i: usize = 1;
    while (i < x.len()) {
        if x[i] > x[max_index] {
            max_index = i;
        }
        i = i + 1;
    }
    max_index
}

}