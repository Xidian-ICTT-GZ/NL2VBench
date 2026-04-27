use vstd::prelude::*;

verus! {

/// Checks whether all characters in the string `s` are the same.
fn all_characters_same(s: &[u8]) -> (result: bool)
    ensures
        result ==> forall|i: int, j: int| 0 <= i < s.len() && 0 <= j < s.len() ==> s[i] == s[j],
        !result ==> (s.len() > 1) && (exists|i: int, j: int| 0 <= i < s.len() && 0 <= j < s.len() && i != j && s[i] != s[j]),
{
    if s.len() <= 1 {
        return true;
    }

    let first_char = s[0];
    let mut result = true;
    let mut i: usize = 1;

    while i < s.len()
        invariant
            1 <= i && i <= s.len(),
            result ==> forall|k: int| 0 <= k && k < i as int ==> s[k as int] == first_char,
            !result ==> exists|k: int| 0 <= k && k < i as int && s[k as int] != first_char,
        decreases 
            s.len() - i
    {
        if s[i] != first_char {
            result = false;
        }
        i += 1;
    }

    result
}

fn main() {}

}