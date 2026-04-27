use vstd::prelude::*;
verus! {
fn fib(n: u32) -> (ret: Option<u32>){
    if n > 47 {
        return None;
    }
    match n {
        0 => Some(0),
        1 => Some(1),
        _ => {
            let n1 = fib(n - 1)?;
            let n2 = fib(n - 2)?;
            Some(n1 + n2)
        },
    }
}
} 