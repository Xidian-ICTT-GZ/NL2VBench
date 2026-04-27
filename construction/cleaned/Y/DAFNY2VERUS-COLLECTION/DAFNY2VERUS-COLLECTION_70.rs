use vstd::prelude::*;
use vstd::seq::*;

verus! {

// Function: append
// Appends an integer `b` to the end of array `a` and returns a new array `c`
fn append(a: Vec<int>, b: int) -> (c: Vec<int>)
    ensures
        a@.add(seq![b]) =~= c@,
{
    let mut c: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            forall|ii: int| 0 <= ii && ii < i as int ==> c[ii] == a[ii],
            c@ =~= a@.take(i as int),
        decreases
            a.len() - i
    {
        c.push(a[i]);
        i += 1;
    }

    c.push(b);
    c
}

// Required empty main function
fn main() {}

}