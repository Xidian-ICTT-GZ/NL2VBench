use vstd::prelude::*;
use vstd::string::*;
verus! {
fn solve(n: i8) -> (result: String){
    if n % 2 == 1 {
        String::from_str("black\n")
    } else {
        String::from_str("white\n1 2\n")
    }
}
}
fn main() {}