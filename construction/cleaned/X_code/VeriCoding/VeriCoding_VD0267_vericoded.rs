use vstd::prelude::*;
verus! {
fn flip(a: &mut Vec<i32>, num: usize){
    let mut i = 0;
    while i <= num / 2
    {
        let temp = a[i];
        let temp2 = a[num - i];
        a.set(i, temp2);
        a.set(num - i, temp);
        i += 1;
    }
}
fn main() {}
}