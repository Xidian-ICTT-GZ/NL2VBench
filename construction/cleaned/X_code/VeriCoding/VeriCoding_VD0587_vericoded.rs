use vstd::prelude::*;
verus! {
fn get_triple(a: &[int]) -> (index: usize){
    if a.len() < 3 {
        return a.len();
    }
    let mut index = 0;
    while index < a.len() - 2
    {
        if a[index] == a[index + 1] && a[index + 1] == a[index + 2] {
            return index;
        }
        index += 1;
    }
    a.len()
}
fn main() {}
}