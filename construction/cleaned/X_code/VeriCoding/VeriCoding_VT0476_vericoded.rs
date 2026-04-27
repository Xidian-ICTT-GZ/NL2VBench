use vstd::prelude::*;
verus! {
fn default_value(c: &Vec<Vec<f32>>) -> (v: f32){
    if c.len() == 1 {
        if c[0].len() == 1 {
            c[0][0]
        } else {
            0.0
        }
    } else {
        0.0
    }
}
fn lagval2d(x: Vec<f32>, y: Vec<f32>, c: Vec<Vec<f32>>) -> (result: Vec<f32>){
    let val = default_value(&c);
    let mut res: Vec<f32> = Vec::new();
    while res.len() < x.len()
    {
        res.push(val);
    }
    res
}
}
fn main() {}