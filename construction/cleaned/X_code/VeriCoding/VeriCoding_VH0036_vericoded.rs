use vstd::prelude::*;
verus! {
fn count7(x: u8) -> (count: u8){
    let mut n: u8 = x;
    let mut c: u8 = 0;
    while n > 0
    {
        let old_n = n;
        let old_c = c;
        if old_n % 10u8 == 7u8 {
            c = old_c + 1;
        } else {
            c = old_c;
        }
        n = old_n / 10u8;
    }
    c
}
}
fn main() {}