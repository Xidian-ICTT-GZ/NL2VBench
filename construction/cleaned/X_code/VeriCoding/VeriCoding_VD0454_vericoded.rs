use vstd::prelude::*;
verus! {
fn get_mini(a: &[i32]) -> (mini: usize){
    let mut mini: usize = 0;
    let mut min_val: i32 = a[0];
    let mut i: usize = 1;
    while i < a.len()
    {
        if a[i] < min_val {
            min_val = a[i];
            mini = i;
        }
        i = i + 1;
    }
    mini
}
fn main() {
}
}