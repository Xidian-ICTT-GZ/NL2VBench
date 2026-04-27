use vstd::prelude::*;
verus! {
fn delete(line: &mut Vec<char>, l: usize, at: usize, p: usize){
    let mut i = at;
    while i < l - p
    {
        line[i] = line[i + p];
        i = i + 1;
    }
    line.truncate(l - p);
}
}
fn main() {}