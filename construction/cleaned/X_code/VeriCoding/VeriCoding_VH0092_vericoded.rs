use vstd::prelude::*;
verus! {
fn rolling_max(s: &Vec<i8>) -> (res: Option<i8>){
    if s.len() < 2 {
        return None;
    }
    let mut max1: i8 = s[0];
    let mut max2: i8 = s[1];
    let mut j1: usize = 0;
    let mut j2: usize = 1;
    if max2 > max1 {
        let tmp = max1;
        max1 = max2;
        max2 = tmp;
        j1 = 1;
        j2 = 0;
    } else {
        j1 = 0;
        j2 = 1;
    }
    let mut i: usize = 2;
    while i < s.len()
    {
        let v = s[i];
        if v >= max1 {
            max2 = max1;
            max1 = v;
            j2 = j1;
            j1 = i;
        } else if v > max2 {
            max2 = v;
            j2 = i;
        } else {
        }
        i = i + 1;
    }
    Some(max2)
}
}
fn main() {}