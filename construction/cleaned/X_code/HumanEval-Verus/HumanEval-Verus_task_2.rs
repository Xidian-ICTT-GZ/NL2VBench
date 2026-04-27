use vstd::prelude::*;
verus! {
fn separate_paren_groups(input: &Vec<char>) -> (groups: Vec<Vec<char>>){
    let mut groups: Vec<Vec<char>> = Vec::new();
    let mut current_group: Vec<char> = Vec::new();
    let input_len = input.len();
    let mut current_nesting_level: usize = 0;
    for pos in 0..input_len
    {
        let c = input[pos];
        if c == '(' {
            current_nesting_level = current_nesting_level + 1;
            current_group.push('(');
        } else if c == ')' {
            current_nesting_level = current_nesting_level - 1;
            current_group.push(')');
            if current_nesting_level == 0 {
                groups.push(current_group);
                current_group = Vec::<char>::new();
            }
        }
    }
    groups
}
} 