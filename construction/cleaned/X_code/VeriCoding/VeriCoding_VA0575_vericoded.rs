use vstd::prelude::*;
verus! {
fn days_to_next_sunday(day: &str) -> (result: i8){
    let result = match day {
        "SUN" => 7i8,
        "MON" => 6i8,
        "TUE" => 5i8,
        "WED" => 4i8,
        "THU" => 3i8,
        "FRI" => 2i8,
        "SAT" => 1i8,
        _ => 1i8,
    };
    result
}
}
fn main() {}