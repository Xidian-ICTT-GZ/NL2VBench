use vstd::prelude::*;
verus! {
fn solve(n: usize, s: Vec<char>) -> (result: Vec<char>){
    let mut sf_count: usize = 0;
    let mut fs_count: usize = 0;
    let mut i: usize = 1;
    while i < n
    {
        let old_sf = sf_count;
        let old_fs = fs_count;
        if s[i] == 'F' && s[i-1] != 'F' {
            sf_count = sf_count + 1;
        }
        if s[i] == 'S' && s[i-1] != 'S' {
            fs_count = fs_count + 1;
        }
        i = i + 1;
    }
    if sf_count > fs_count {
        vec!['Y', 'E', 'S']
    } else {
        vec!['N', 'O']
    }
}
}
fn main() {}