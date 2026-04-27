use vstd::prelude::*;

verus! {

fn reverse(a: &Vec<i32>) -> (a_rev: Vec<i32>)
    ensures
        a_rev.len() == a.len(),
        forall|i: int| 0 <= i < a.len() ==> a[i] == a_rev[a_rev.len() - i - 1]
// </vc-spec>
// <vc-code>
{
    let mut a_rev = Vec::with_capacity(a.len());
    let mut i = 0;
    
    while i < a.len()
        invariant
            a_rev.len() == i,
            i <= a.len(),
            forall|j: int| 0 <= j < i ==> a[a.len() - j - 1] == a_rev[j]
        decreases a.len() - i
    {
        a_rev.push(a[a.len() - i - 1]);
        i += 1;
    }
    
    a_rev
}
// </vc-code>

fn main() {}

}