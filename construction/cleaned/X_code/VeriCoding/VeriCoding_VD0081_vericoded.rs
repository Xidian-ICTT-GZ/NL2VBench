use vstd::prelude::*;
verus! {
fn reverse(a: &mut Vec<i32>){
    let n = a.len();
    let mut i = 0;
    while i < n / 2
    {
        let temp = a[i];
        let other_temp = a[n - 1 - i];
        a.set(i, other_temp);
        a.set(n - 1 - i, temp);
        i += 1;
    }
}
fn main() {
}
}