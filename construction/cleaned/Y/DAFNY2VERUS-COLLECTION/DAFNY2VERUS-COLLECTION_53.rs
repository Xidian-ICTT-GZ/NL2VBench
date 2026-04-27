use vstd::prelude::*;
use vstd::seq::*;
use vstd::map::*;

verus! {

/// Reverses the input array of characters and returns a new array.
fn reverse(a: Vec<char>) -> (b: Vec<char>)
    requires
        a.len() > 0,
    ensures
        a == a,  // old(a) is not needed since a is not mutated; this is trivially true
        b.len() == a.len(),
        forall|i: int| 0 <= i < a.len() ==> b[i] == a[a.len() - i - 1],
{
    let mut b: Vec<char> = Vec::new();
    let mut i: usize = 0;

    while i < a.len()
        invariant
            0 <= i && i <= a.len(),
            b.len() == i,
            forall|j: int| 0 <= j < i as int ==> b[j] == a[a.len() - j - 1],
         decreases 
            a.len() - i 
    {
        b.push(a[a.len() - i - 1]);
        i += 1;
    }

    b
}

} // verus!

// We can now write a non-verified main function that uses the verified logic
// Note: In Verus, `main` cannot be verified yet due to IO restrictions, but we define it normally.

fn main() {
    // Simulate the first test case
    let a = vec!['s', 'k', 'r', 'o', 'w', 't', 'i'];
    let b = reverse(a);
    assert_eq!(b.as_slice(), &['i', 't', 'w', 'o', 'r', 'k', 's']);
    println!("{:?}", b);

    // Second test case
    let a = vec!['!'];
    let b = reverse(a);
    assert_eq!(b.as_slice(), &['!']);
    println!("{}", b.iter().collect::<String>());
}