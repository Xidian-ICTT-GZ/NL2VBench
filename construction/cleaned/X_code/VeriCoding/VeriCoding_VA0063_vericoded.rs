use vstd::prelude::*;
verus! {
fn shell_game(n: i32, x: i8) -> (result: i8){
    if n % 2 == 0 {
        let t: i32 = (n / 2) % 3;
        let res: i8 = if t == 0 {
            x
        } else if t == 1 {
            if x == 0 { 2 } else if x == 1 { 0 } else { 1 }
        } else {
            if x == 0 { 1 } else if x == 1 { 2 } else { 0 }
        };
        res
    } else {
        let t: i32 = ((n - 1) / 2) % 3;
        let y: i8 = if x == 0 { 1 } else if x == 1 { 0 } else { 2 };
        let res: i8 = if t == 0 {
            y
        } else if t == 1 {
            if y == 0 { 1 } else if y == 1 { 2 } else { 0 }
        } else {
            if y == 0 { 2 } else if y == 1 { 0 } else { 1 }
        };
        res
    }
}
}
fn main() {}