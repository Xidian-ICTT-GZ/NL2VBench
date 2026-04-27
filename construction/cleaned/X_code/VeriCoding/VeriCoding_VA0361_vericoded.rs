use vstd::prelude::*;
verus! {
fn solve(n: i8, s: Vec<char>) -> (result: i8){
    let mut pos: i8 = 0;
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut pred: i8 = -1;
    let mut transitions: i8 = 0;
    while pos < n
    {
        let char = s[pos as usize];
        let new_x = if char == 'U' { x } else { x + 1 };
        let new_y = if char == 'U' { y + 1 } else { y };
        if new_x != new_y {
            let cur = if new_x > new_y { 0 } else { 1 };
            if cur != pred && pred != -1 {
                transitions = transitions + 1;
            }
            pred = cur;
        }
        x = new_x;
        y = new_y;
        pos = pos + 1;
    }
    transitions
}
}
fn main() {}