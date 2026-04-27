use vstd::prelude::*;
verus! {
fn fib(n: u32) -> (ret: Option<u32>){
    if n == 0 {
        return Some(0);
    }
    if n == 1 {
        return Some(1);
    }
    if n > 47 {
        return None;
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut i: u32 = 2;
    for i in 1..n
    {
        let sum = a + b;
        a = b;
        b = sum;
    }
    Some(b)
}
} 