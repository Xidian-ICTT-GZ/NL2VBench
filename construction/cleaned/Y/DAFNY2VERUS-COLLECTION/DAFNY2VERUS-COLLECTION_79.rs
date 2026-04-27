use vstd::prelude::*;

verus! {

/// Finds the index of the first occurrence of `'e'` in the character array.
/// Returns -1 if `'e'` is not present.
fn first_e(a: &[char]) -> (x: i32)
    requires
        a.len() < i32::MAX as usize,
    ensures
        if exists|i: int| 0 <= i < a.len() && a[i] == 'e' {
            0 <= x && x < a.len() as i32 && a[x as int] == 'e' &&
            forall|i: int| 0 <= i < x as int ==> a[i] != 'e'
        } else {
            x == -1
        },
{
    let mut i: i32 = 0;

    while i < a.len() as i32
        invariant
            0 <= i && i <= a.len() as i32,
            forall|j: int| 0 <= j < i ==> a[j as int] != 'e',
        decreases 
            a.len() - i 
    {
        if a[i as usize] == 'e' {
            return i;
        }
        i += 1;
    }

    -1
}

} // end verus!

fn main() {}