use vstd::prelude::*;

verus! {

/// Extracts the first element of each inner sequence from a sequence of sequences.
fn get_first_elements(lst: Vec<Vec<int>>) -> (result: Vec<int>)
    requires
        forall|i: int| 0 <= i < lst.len() ==> lst[i].len() > 0,
    ensures
        result.len() == lst.len(),
        forall|i: int| 0 <= i < result.len() ==> result[i] == lst[i][0],
{
    let mut result: Vec<int> = Vec::new();
    let mut i: usize = 0;

    while i < lst.len()
        invariant
            0 <= i && i <= lst.len(),
            result.len() == i,
            forall|j: int| 0 <= j < i ==> result[j as int] == lst[j as int][0],
            forall|j: int| 0 <= j < lst.len() ==> lst[j as int].len() > 0,
        decreases 
            lst.len() - i
    {
        result.push(lst[i][0]);
        i += 1;
    }

    result
}

fn main() {}

}