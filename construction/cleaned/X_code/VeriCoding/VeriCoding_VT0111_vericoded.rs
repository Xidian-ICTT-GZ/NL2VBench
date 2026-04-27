use vstd::prelude::*;
verus! {

fn infinity_f32() -> (r: f32)
{
    1.0e39f32
}

fn inf() -> (result: f32)
{
    let result: f32 = infinity_f32();
    result
}

}
fn main() {}