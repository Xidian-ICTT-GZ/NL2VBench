use vstd::prelude::*;
verus! {
fn triangle_area(a: u64, h: u64) -> (area: u64){
    if a % 2 == 0 {
        (a / 2) * h
    } else {
        (a / 2) * h + (h / 2)
    }
}
} 