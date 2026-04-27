use vstd::prelude::*;
verus! {
fn cube_root(n: u8) -> (r: u8){
    if n < 1u8 {
        0u8
    } else if n < 8u8 {
        1u8
    } else if n < 27u8 {
        2u8
    } else if n < 64u8 {
        3u8
    } else if n < 125u8 {
        4u8
    } else if n < 216u8 {
        5u8
    } else {
        6u8
    }
}
}
fn main() {}