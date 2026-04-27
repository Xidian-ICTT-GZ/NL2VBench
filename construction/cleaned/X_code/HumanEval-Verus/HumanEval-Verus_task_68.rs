use vstd::prelude::*;
verus! {
fn monotonic(l: Vec<i32>) -> (ret: bool){
    if l.len() == 0 || l.len() == 1 {
        return true;
    }
    let mut increasing = true;
    let mut decreasing = true;
    let mut n = 0;
    while n < l.len() - 1
    {
        if l[n] < l[n + 1] {
            decreasing = false;
        } else if l[n] > l[n + 1] {
            increasing = false;
        }
        n += 1;
    }
    increasing || decreasing
}
} 