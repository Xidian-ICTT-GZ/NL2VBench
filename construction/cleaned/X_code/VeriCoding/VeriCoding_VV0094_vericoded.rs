use vstd::prelude::*;
verus! {
fn contains_z(s: &str) -> (result: bool){
    let mut i: usize = 0;
    let mut found = false;
    let len = s.unicode_len();
    while i < len
    {
        let c = s.get_char(i);
        if c == 'z' || c == 'Z' {
            found = true;
        }
        i = i + 1;
    }
    found
}
}
fn main() {}