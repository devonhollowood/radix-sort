use std::collections::VecDeque;
use std::mem;

pub fn lsd_radix_sort(input: &mut [u64]) {
    let mut buckets = vec![VecDeque::<u64>::new(); 16];
    for nibble_shift in 0 .. mem::size_of::<u64>()*2 {
        for elem in input.iter() {
            let bin = ((elem >> nibble_shift * 4) & 0xF) as usize;
            buckets[bin].push_back(elem.clone());
        }
        for (idx, elem) in buckets.iter_mut().flat_map(|v| v.drain(..)).enumerate() {
            input[idx] = elem;
        }
    }
}
