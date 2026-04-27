use vstd::prelude::*;
verus! {
fn any_int(x: i8, y: i8, z: i8) -> (result: bool){
    let xi128: i128 = x as i128;
    let yi128: i128 = y as i128;
    let zi128: i128 = z as i128;
    let b: bool = xi128 == yi128 + zi128
        || yi128 == xi128 + zi128
        || zi128 == xi128 + yi128;
    b
}
}
fn main() {}