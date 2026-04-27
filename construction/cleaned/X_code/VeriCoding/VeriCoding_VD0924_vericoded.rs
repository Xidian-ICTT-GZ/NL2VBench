use vstd::prelude::*;
verus! {
fn torneo(valores: &Vec<int>, i: usize, j: usize, k: usize) -> (res: (usize, usize)){
    let vi = valores[i];
    let vj = valores[j];
    let vk = valores[k];
    if vi >= vj && vj >= vk {
        let p0 = i;
        let q0 = j;
        let r0 = k;
        let res_ = (p0, q0);
        res_
    } else if vi >= vk && vk >= vj {
        let p0 = i;
        let q0 = k;
        let r0 = j;
        let res_ = (p0, q0);
        res_
    } else if vj >= vi && vi >= vk {
        let p0 = j;
        let q0 = i;
        let r0 = k;
        let res_ = (p0, q0);
        res_
    } else if vj >= vk && vk >= vi {
        let p0 = j;
        let q0 = k;
        let r0 = i;
        let res_ = (p0, q0);
        res_
    } else if vk >= vi && vi >= vj {
        let p0 = k;
        let q0 = i;
        let r0 = j;
        let res_ = (p0, q0);
        res_
    } else {
        let p0 = k;
        let q0 = j;
        let r0 = i;
        let res_ = (p0, q0);
        res_
    }
}
fn main() {}
}