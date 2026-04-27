use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: concat
// Concatenates two arrays `a` and `b` into a new array `c`
fn concat(a: Vec<int>, b: Vec<int>) -> (c: Vec<int>)
    ensures
        c.len() == b.len() + a.len(),
        forall|k: int| 0 <= k < a.len() ==> c[k] == a[k],
        forall|k: int| 0 <= k < b.len() ==> c[k + a.len() as int] == b[k],
{
    let mut c: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i as int && i as int <= a.len(),
            c.len() == i,
            forall|k: int| 0 <= k < i as int ==> c[k] == a[k],
        decreases
            a.len() - i
    {
        c.push(a[i]);
        i += 1;
    }

    i = 0;
    while i < b.len()
        invariant
            0 <= i as int && i as int <= b.len(),
            c.len() == a.len() + i,
            forall|k: int| 0 <= k < a.len() ==> c[k] == a[k],
            forall|k: int| 0 <= k < i as int ==> c[k + a.len() as int] == b[k],
        decreases
            b.len() - i
    {
        c.push(b[i]);
        i += 1;
    }

    c
}

// Required empty main function
fn main() {}

}