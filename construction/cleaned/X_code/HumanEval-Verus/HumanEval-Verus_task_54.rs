use vstd::prelude::*;
verus! {
fn sort_third(l: Vec<i32>) -> (l_prime: Vec<i32>){
    let l_len = l.len();
    let mut pos_being_set_to_smallest: usize = 0;
    let mut l_prime: Vec<i32> = l;
    while pos_being_set_to_smallest < l_len
    {
        let mut pos_of_smallest_found_so_far: usize = pos_being_set_to_smallest;
        let mut pos_during_scan_for_smallest: usize = pos_being_set_to_smallest;
        while pos_during_scan_for_smallest < l_len
        {
            if l_prime[pos_during_scan_for_smallest] < l_prime[pos_of_smallest_found_so_far] {
                pos_of_smallest_found_so_far = pos_during_scan_for_smallest;
            }
            pos_during_scan_for_smallest = pos_during_scan_for_smallest + 3;
        }
        let v1 = l_prime[pos_being_set_to_smallest];
        let v2 = l_prime[pos_of_smallest_found_so_far];
        l_prime.set(pos_being_set_to_smallest, v2);
        l_prime.set(pos_of_smallest_found_so_far, v1);
        pos_being_set_to_smallest = pos_being_set_to_smallest + 3;
    }
    l_prime
}
} 