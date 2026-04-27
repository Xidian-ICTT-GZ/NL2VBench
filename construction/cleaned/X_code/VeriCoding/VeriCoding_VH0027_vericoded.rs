use vstd::prelude::*;
verus! {
fn has_prefix(v: &Vec<char>, p: &Vec<char>) -> (b: bool){
    let mut i: usize = 0;
    while i < p.len()
    {
        if i >= v.len() {
            return false;
        }
        let vc = v[i];
        let pc = p[i];
        if vc != pc {
            return false;
        }
        i = i + 1;
    }
    true
}
fn filter_by_prefix(xs: Vec<Vec<char>>, p: Vec<char>) -> (filtered: Vec<Vec<char>>){
    let mut filtered: Vec<Vec<char>> = Vec::new();
    let mut i: usize = 0;
    while i < xs.len()
    {
        let v_keep = xs[i].clone();
        let b = has_prefix(&v_keep, &p);
        i = i + 1;
        if b {
            let to_push = v_keep.clone();
            filtered.push(to_push);
        }
    }
    filtered
}
}
fn main() {}