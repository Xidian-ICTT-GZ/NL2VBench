use vstd::prelude::*;

verus! {

/// Searches for the first occurrence of `key` in the array `blood`.
/// Returns the index if found, otherwise returns -1.
fn find(blood: Vec<i32>, key: i32) -> (index: i32)
    requires
        blood.len() < i32::MAX as usize,
    ensures
        0 <= index ==> index < blood.len() as i32 && blood[index as int] == key,
        index < 0 ==> forall|k: int| 0 <= k && k < blood.len() ==> blood[k] != key,
{
    let mut index: i32 = 0;

    while index < blood.len() as i32
        invariant
            0 <= index && index <= blood.len() as i32,
            forall|k: int| 0 <= k && k < index ==> blood[k] != key,
        decreases
            blood.len() - index
    {
        if blood[index as usize] == key {
            return index;
        }
        index += 1;
    }

    -1
}

fn main() {}

}