use vstd::prelude::*;
use vstd::slice::*;

verus! {

fn string_xor(a: &[char], b: &[char]) -> (result: Vec<char>)
    requires
        a.len() == b.len(),
        forall|i: int| 0 <= i < a.len() ==> a[i] == '0' || a[i] == '1',
        forall|i: int| 0 <= i < b.len() ==> b[i] == '0' || b[i] == '1',
    ensures
        result.len() == a.len(),
        forall|i: int| 0 <= i < result.len() ==> 
            (a[i] == b[i] ==> result[i] == '0') && 
            (a[i] != b[i] ==> result[i] == '1'),
        forall|i: int| 0 <= i < result.len() ==> result[i] == '0' || result[i] == '1',
{
    let a_len = a.len();
    let mut result = Vec::with_capacity(a_len);
    let mut pos: usize = 0;
    while pos < a_len
        invariant
            a_len == a.len(),
            a_len == b.len(),
            0 <= pos <= a_len,
            result.len() == pos,
            forall|i: int| 0 <= i < pos ==> 
                (a[i] == b[i] ==> result[i] == '0') && 
                (a[i] != b[i] ==> result[i] == '1'),
            forall|i: int| 0 <= i < pos ==> result[i] == '0' || result[i] == '1',
        decreases a_len - pos
    {
        if *slice_index_get(a, pos) == *slice_index_get(b, pos) {
            result.push('0');
        } else {
            result.push('1');
        }
        pos += 1;
    }
    result
}

}
