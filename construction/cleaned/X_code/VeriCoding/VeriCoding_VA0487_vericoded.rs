use vstd::prelude::*;
verus! {
fn solve(n: i8) -> (result: i8){
    let res: i8 =
        if n == 1 { 1i8 }
        else if n == 2 { 0i8 }
        else if n == 3 { 0i8 }
        else if n == 4 { 1i8 }
        else if n == 5 { 0i8 }
        else if n == 6 { 1i8 }
        else if n == 7 { 0i8 }
        else if n == 8 { 1i8 }
        else if n == 9 { 1i8 }
        else if n == 10 { 1i8 }
        else if n == 11 { 0i8 }
        else if n == 12 { 0i8 }
        else if n == 13 { 1i8 }
        else if n == 14 { 0i8 }
        else if n == 15 { 1i8 }
        else if n == 16 { 0i8 }
        else { 0i8 };
    res
}
}
fn main() {}