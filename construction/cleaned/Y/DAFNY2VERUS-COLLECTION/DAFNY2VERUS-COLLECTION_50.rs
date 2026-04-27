use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Removes the first element of the array and returns a new array with the remaining elements.
fn remove_front(a: Vec<i32>) -> (c: Vec<i32>)
    requires
        a.len() > 0,
    ensures
        a@.subrange(1, a.len() as int) =~= c@,
{
    let mut c: Vec<i32> = Vec::new();
    let mut i: usize = 1;

    while i < a.len()
        invariant
            1 <= i && i <= a.len(),
            forall|ii: int| 1 <= ii && ii < i ==> c[ii - 1] == a[ii as int],
            c@ =~= a@.subrange(1, i as int),
        decreases 
            a.len() - i
    {
        c.push(a[i]);
        i += 1;
    }

    c
}

fn main() {}

}