use vstd::prelude::*;
fn main() {}
verus!{

pub fn myfun(a: &mut Vec<i32>, sum: &mut Vec<i32>, N: i32){
	let mut i: usize = 0;
	while (i < N as usize)
	{
		a.set(i, (i % 3) as i32);
		i = i + 1;
	}

	i = 0;

	while (i < N as usize)
	{
		if (i == 0) {
			sum.set(0, 0);
		} else {
			sum.set(0, sum[0] + a[i]);
		}
		i = i + 1;
	}
}
}