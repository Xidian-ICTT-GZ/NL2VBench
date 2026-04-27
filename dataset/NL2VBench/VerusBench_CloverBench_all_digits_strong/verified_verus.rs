use vstd::prelude::*;

fn main() {}

verus! {

spec fn is_ascii_digit_spec(c: char) -> bool {
    c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7'
        || c == '8' || c == '9'
}

fn is_ascii_digit(c: char) -> (r: bool)
    ensures
        r == is_ascii_digit_spec(c),
{
    c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7'
        || c == '8' || c == '9'
}

fn all_digits(s: String) -> (result: bool)
    ensures
        result == (forall|i: int| 0 <= i < s@.len() ==> is_ascii_digit_spec(s@[i])),
{
    let mut result = true;
    let mut i = 0;
    while i < s.as_str().unicode_len()
        invariant
            0 <= i <= s@.len(),
            result == (forall|j: int| 0 <= j < i ==> is_ascii_digit_spec(s@[j])),
        decreases s@.len() - i
    {
        if !is_ascii_digit(s.as_str().get_char(i)) {
            return false;
        }
        i += 1;
    }
    true
}

} // verus!