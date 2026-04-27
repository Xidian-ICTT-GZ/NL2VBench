use vstd::prelude::*;
verus! {
fn extract_rear_chars(l: &Vec<Vec<char>>) -> (r: Vec<char>){
    let mut r = Vec::new();
    let mut i: usize = 0;
    while i < l.len()
    {
        let last_char = l[i][l[i].len() - 1];
        r.push(last_char);
        i = i + 1;
    }
    r
}
}
fn main() {}