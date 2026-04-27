use vstd::prelude::*;
verus! {
fn solve(x: i8) -> (result: u8){
    let angle_i32: i32 = ((x as i32) % 360 + 360) % 360;
    let dev0: i32 = if angle_i32 <= 180 { angle_i32 } else { 360 - angle_i32 };
    let dev1: i32 = if angle_i32 <= 90 { 90 - angle_i32 } else if angle_i32 <= 270 { angle_i32 - 90 } else { 450 - angle_i32 };
    let dev2: i32 = if angle_i32 <= 180 { 180 - angle_i32 } else { angle_i32 - 180 };
    let dev3: i32 = if angle_i32 <= 90 { angle_i32 + 90 } else if angle_i32 <= 270 { 270 - angle_i32 } else { angle_i32 - 270 };
    let mut result = 0u8;
    let mut min_dev = dev0;
    if dev1 < min_dev {
        min_dev = dev1;
        result = 1;
    }
    if dev2 < min_dev {
        min_dev = dev2;
        result = 2;
    }
    if dev3 < min_dev {
        min_dev = dev3;
        result = 3;
    }
    result
}
}
fn main() {}