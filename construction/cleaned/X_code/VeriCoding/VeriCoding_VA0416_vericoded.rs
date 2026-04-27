use vstd::prelude::*;
verus! {
pub enum Result {
    Impossible,
    Possible { cost: int, edges: Seq<(int, int)> }
}
fn solve(t: i8, cases: Vec<(i8, i8, Vec<i8>)>) -> (results: Vec<Result>){
    let n: usize = cases.len();
    let mut res: Vec<Result> = Vec::new();
    let mut i: usize = 0;
    while i < n
    {
        res.push(Result::Impossible);
        i += 1;
    }
    res
}
}
fn main() {}