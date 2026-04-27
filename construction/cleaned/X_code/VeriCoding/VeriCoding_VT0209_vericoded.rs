use vstd::prelude::*;
verus! {
fn mk_nanstr() -> (v: Vec<char>){
    let mut v = Vec::<char>::new();
    v.push('n');
    v.push('a');
    v.push('n');
    v
}
fn mk_infstr() -> (v: Vec<char>){
    let mut v = Vec::<char>::new();
    v.push('i');
    v.push('n');
    v.push('f');
    v
}
fn mk_sign_minus() -> (v: Vec<char>){
    let mut v = Vec::<char>::new();
    v.push('-');
    v
}
fn mk_floatmode_fixed() -> (v: Vec<char>){
    let mut v = Vec::<char>::new();
    v.push('f');
    v.push('i');
    v.push('x');
    v.push('e');
    v.push('d');
    v
}
struct PrintOptions {
    precision: u8,
    threshold: u8,
    edgeitems: u8,
    linewidth: u8,
    suppress: bool,
    nanstr: Vec<char>,
    infstr: Vec<char>,
    sign: Vec<char>,
    floatmode: Vec<char>,
    legacy: Option<Vec<char>>,
}
fn get_printoptions() -> (result: PrintOptions){
    let precision: u8 = 3;
    let threshold: u8 = 10;
    let edgeitems: u8 = 2;
    let linewidth: u8 = 80;
    let suppress: bool = false;
    let nanstr = mk_nanstr();
    let infstr = mk_infstr();
    let sign = mk_sign_minus();
    let floatmode = mk_floatmode_fixed();
    let legacy: Option<Vec<char>> = None;
    let res = PrintOptions {
        precision,
        threshold,
        edgeitems,
        linewidth,
        suppress,
        nanstr,
        infstr,
        sign,
        floatmode,
        legacy,
    };
    res
}
}
fn main() {}