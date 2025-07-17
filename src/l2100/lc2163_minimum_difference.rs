pub fn minimum_difference(mut nums: Vec<i32>) -> i64 {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let length = nums.len();
    assert_eq!(length % 3, 0, "The length of nums must be a multiple of 3");
    let n = length / 3;

    let mut max_heap = BinaryHeap::with_capacity(n);
    let mut min_heap = BinaryHeap::with_capacity(n);
    let mut sum = 0;
    for i in 2 * n..length {
        min_heap.push(Reverse(nums[i] as i64));
        sum += nums[i] as i64;
    }
    let mut suffix_max = vec![0; 2 * n + 1];
    suffix_max[2 * n] = sum;
    for i in (n..n * 2).rev() {
        let cur = nums[i] as i64;
        min_heap.peek_mut().map(|mut val| {
            if cur > (*val).0 {
                sum += cur - (*val).0;
                *val = Reverse(cur);
            }
        });
        suffix_max[i] = sum;
    }

    let mut prefix_min: i64 = 0;
    for i in 0..n {
        let cur = nums[i] as i64;
        max_heap.push(cur);
        prefix_min += cur;
    }

    let mut ans = prefix_min - suffix_max[n];
    for i in n..n * 2 {
        let cur = nums[i] as i64;
        max_heap.peek_mut().map(|mut val| {
            if cur < *val {
                prefix_min += cur - *val;
                *val = cur;
            }
        });
        ans = ans.min(prefix_min - suffix_max[i + 1]);
    }

    ans as i64
}
