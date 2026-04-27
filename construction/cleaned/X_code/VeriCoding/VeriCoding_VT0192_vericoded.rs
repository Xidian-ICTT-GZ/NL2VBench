use vstd::prelude::*;
verus! {
pub struct Slice {
    pub start: Option<usize>,
    pub stop: Option<usize>,
    pub step: Option<usize>,
}
fn s_(start: Option<usize>, stop: Option<usize>, step: Option<usize>) -> (slice: Slice){
    Slice {
        start: start,
        stop: stop,
        step: step,
    }
}
}
fn main() {}