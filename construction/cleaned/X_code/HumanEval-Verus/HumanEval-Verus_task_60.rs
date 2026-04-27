use vstd::prelude::*;
verus! {
fn is_vowel(c: char) -> (is_vowel: bool){
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'A' || c == 'E' || c == 'I'
        || c == 'O' || c == 'U'
}
fn remove_vowels(str: &[char]) -> (str_without_vowels: Vec<char>){
    let mut str_without_vowels: Vec<char> = Vec::new();
    for index in 0..str.len()
    {
        if !is_vowel(str[index]) {
            str_without_vowels.push(str[index]);
        }
    }
    str_without_vowels
}
} 