use vstd::prelude::*;
verus! {
fn has_common_element(a: &Vec<i32>, b: &Vec<i32>) -> (result: bool){
    let mut i: usize = 0;
    while i < a.len()
    {
        let mut j: usize = 0;
        while j < b.len()
        {
            if a[i] == b[j] {
                return true;
            }
            j = j + 1;
        }
        i = i + 1;
    }
    false
}
}
fn main() {}