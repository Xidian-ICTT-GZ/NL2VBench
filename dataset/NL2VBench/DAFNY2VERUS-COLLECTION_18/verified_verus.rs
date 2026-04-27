use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Appends two arrays `a` and `b` into a new array `c`
fn append_array(a: Vec<i32>, b: Vec<i32>) -> (c: Vec<i32>)
    ensures
        c.len() == a.len() + b.len(),
        forall|i: int| 0 <= i < a.len() ==> a[i as int] == c[i],
        forall|i: int| 0 <= i < b.len() ==> b[i as int] == c[a.len() as int + i],
{
    let mut c: Vec<i32> = Vec::new();
    let mut i: usize = 0;

    // First loop: copy all elements from `a` into `c`
    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            c.len() == i,
            forall|j: int| 0 <= j < i as int ==> c[j] == a[j],
        decreases 
            a.len() - i
    {
        c.push(a[i]);
        i += 1;
    }

    i = 0;
    // Second loop: copy all elements from `b` into `c`
    while i < b.len()
        invariant
            0 <= i && i <= b.len(),
            c.len() == a.len() + i,
            forall|j: int| 0 <= j < a.len() ==> c[j] == a[j],
            forall|j: int| 0 <= j < i as int ==> c[a.len() as int + j] == b[j],
        decreases 
            b.len() - i
    {
        c.push(b[i]);
        i += 1;
    }

    c
}

fn main() {}

}