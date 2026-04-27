use vstd::prelude::*;
verus! {
fn first_e(a: &[char]) -> (x: i32){
    let mut i: i32 = 0;
    while i < a.len() as i32
    {
        if a[i as usize] == 'e' {
            return i;
        }
        i += 1;
    }
    -1
}
} 
fn main() {}