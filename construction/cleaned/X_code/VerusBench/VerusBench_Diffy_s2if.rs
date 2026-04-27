use vstd::prelude::*;
fn main() {}
verus!{
pub fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
	sum.set(0, 0);
	let mut i: usize = 0;
	while (i < N as usize)
	{
		a.set(i, 2);
		i = i + 1;
	}

	i = 0;
	while (i < N as usize)
	{
		if (a[i] == 2) {
			sum.set(0, sum[0] + a[i]);
		} else {
			sum.set(0, sum[0] * a[i]);
		}
		i = i + 1;
	}
}
}