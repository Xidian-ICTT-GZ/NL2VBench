use vstd::prelude::*;
verus! {
pub struct FloatInfo {
    pub eps: u32,              
    pub epsneg: u32,           
    pub max: u32,              
    pub min: i32,              
    pub tiny: u32,             
    pub smallest_subnormal: u32, 
    pub maxexp: i32,           
    pub minexp: i32,           
    pub negep: i32,            
    pub nexp: u32,             
    pub nmant: u32,            
    pub precision: u32,        
}
fn numpy_finfo() -> (info: FloatInfo){
    let max_val: u32 = 100u32;
    let info = FloatInfo {
        eps: 1u32,
        epsneg: 1u32,
        max: max_val,
        min: -(max_val as i32),
        tiny: 2u32,
        smallest_subnormal: 1u32,
        maxexp: 10,
        minexp: -10,
        negep: -1,
        nexp: 8u32,
        nmant: 24u32,
        precision: 6u32,
    };
    info
}
}
fn main() {}