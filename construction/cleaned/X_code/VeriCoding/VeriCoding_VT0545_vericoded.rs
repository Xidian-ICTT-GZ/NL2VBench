use vstd::prelude::*;
verus! {
struct RandomState {
    seed: u8,
}
fn normalize_u8_to_unit_interval(seed: u8) -> f32 {
    let rem: u8 = seed % 8u8;
    match rem {
        0u8 => 0.0f32,
        1u8 => 0.125f32,
        2u8 => 0.25f32,
        3u8 => 0.375f32,
        4u8 => 0.5f32,
        5u8 => 0.625f32,
        6u8 => 0.75f32,
        _ => 0.875f32,
    }
}
fn random(state: RandomState) -> (result: f32)
{
    let result = normalize_u8_to_unit_interval(state.seed);
    result
}
}
fn main() {}