use vstd::prelude::*;
verus! {
fn fibfib(x: u32) -> (ret: Option<u32>){
    if x > 39 {
        return None;
    }
    match (x) {
        0 => Some(0),
        1 => Some(0),
        2 => Some(1),
        _ => {
            Some(fibfib(x - 1)? + fibfib(x - 2)? + fibfib(x - 3)?)
        },
    }
}
} 