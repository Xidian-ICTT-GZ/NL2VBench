use vstd::prelude::*;
verus! {
fn find_median(a: &[i32], b: &[i32]) -> (median: i32){
    let n = a.len();
    if n % 2 == 0 {
        let x = a[n / 2 - 1] as i64;
        let y = b[0] as i64;
        let sum = x + y;
        let avg = if sum >= 0 {
            sum / 2
        } else {
            (sum - 1) / 2
        };
        avg as i32
    } else {
        a[n / 2]
    }
}
}
fn main() {}