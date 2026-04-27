use vstd::prelude::*;

verus! {

/// Checks whether the given string `s` contains a decimal point followed by exactly two digits
/// (i.e., has exactly two decimal places at the end).
fn is_decimal_with_two_precision(s: &[u8]) -> (result: bool)
    ensures
        result ==> exists|i: int| 0 <= i < s.len() && s[i] == '.' as u8 && s.len() - i - 1 == 2,
        !result ==> !(exists|i: int| 0 <= i < s.len() && s[i] == '.' as u8 && s.len() - i - 1 == 2),
{
    let mut result = false;
    let mut i: usize = 0;

    while i < s.len()
        invariant
            0 <= i && i <= s.len(),
            result <==> exists|k: int| 0 <= k && k < i && s[k as int] == '.' as u8 && s.len() - k - 1 == 2,
        decreases 
            s.len() - i
    {
        if s[i] == '.' as u8 && s.len() - i - 1 == 2 {
            result = true;
        }
        i += 1;
    }

    result
}

fn main() {}

}