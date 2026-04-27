use vstd::prelude::*;
verus! {
fn correct_bracketing(brackets: &str) -> (ret: bool){
    let mut i = 0;
    let mut b = true;
    let mut stack_size: i32 = 0;
    while i < brackets.unicode_len()
    {
        let c = brackets.get_char(i);
        if (c == '(') {
            stack_size += 1;
        } else if (c == ')') {
            b = b && stack_size > 0;
            stack_size -= 1;
        }
        i += 1;
    }
    b && stack_size == 0
}
} 