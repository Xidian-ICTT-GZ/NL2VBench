use vstd::prelude::*;
verus! {

fn rjust(orig: Vec<char>, width: u8, fillchar: char) -> (res: Vec<char>){
    let mut orig = orig;
    if orig.len() >= width as usize {
        return orig;
    } else {
        let orig_len: usize = orig.len();
        let pad_len_usize: usize = (width as usize) - orig_len;
        let mut res: Vec<char> = Vec::new();
        let mut i: usize = 0;
        while i < pad_len_usize
        {
            res.push(fillchar);
            i = i + 1;
        }
        res.append(&mut orig);
        res
    }
}
}
fn main() {}