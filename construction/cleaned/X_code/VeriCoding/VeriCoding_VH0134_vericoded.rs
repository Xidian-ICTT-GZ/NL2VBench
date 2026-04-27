use vstd::prelude::*;
verus! {
fn is_nested(s: Vec<i8>) -> (res: bool){
    let n = s.len();
    if n < 4 {
        return false;
    }
    let mut i: usize = 0;
    while i < n - 3
    {
        if s[i] == 0 {
            let mut j: usize = i + 1;
            while j < n - 2
            {
                if s[j] == 0 {
                    let mut k: usize = j + 1;
                    while k < n - 1
                    {
                        if s[k] == 1 {
                            let mut l: usize = k + 1;
                            while l < n
                            {
                                if s[l] == 1 {
                                    return true;
                                }
                                l = l + 1;
                            }
                        }
                        k = k + 1;
                    }
                }
                j = j + 1;
            }
        }
        i = i + 1;
    }
    false
}
}
fn main() {}