use vstd::prelude::*;
verus! {
fn prime_length(str: &[char]) -> (result: bool){
    if str.len() < 2 {
        return false;
    }
    for index in 2..str.len()
    {
        if ((str.len() % index) == 0) {
            return false;
        }
    }
    true
}
} 