use vstd::prelude::*;
use vstd::seq::*;

verus! {

/// Reverses the input array of characters and returns a new array.
fn reverse(a: Vec<char>) -> (b: Vec<char>)
    requires
        a.len() > 0,
    ensures
        a.len() == b.len(),
        forall|x: int| 0 <= x < a.len() ==> b[x] == a[a.len() - x - 1],
{
    let mut b: Vec<char> = Vec::new();
    let mut k: usize = 0;

    while k < a.len()
        invariant
            0 <= k && k <= a.len(),
            b.len() == k,
            forall|x: int| 0 <= x < k as int ==> b[x] == a[a.len() - x - 1],
        decreases a.len() - k
    {
        b.push(a[a.len() - 1 - k]);
        k = k + 1;
    }

    b
}

} // end of verus! block

// Rust-compatible main function to test the logic
fn main() {
    // First test case: "reverser" -> "rerveser" (actually "reverse" backwards is "esrever"?)
    // But Dafny example uses: ['d','e','s','r','e','v','e','r'] -> reversed should be ['r','e','v','e','r','s','e','d']
    let mut a = vec!['d', 'e', 's', 'r', 'e', 'v', 'e', 'r'];
    let b = reverse(a);
    assert_eq!(b.as_slice(), &['r', 'e', 'v', 'e', 'r', 's', 'e', 'd']);
    print!("{:?}", b);

    // Second test case: single '!'
    a = vec!['!'];
    let b = reverse(a);
    assert_eq!(b.as_slice(), &['!']);
    println!("{:?}", b);
}