use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

#[allow(unused)]
impl Solution {
    fn quick_mul(x: i64, y: i64, m: i64) -> i64 {
        let mut res = 1;
        let mut x = x;
        let mut y = y;
        while y > 0 {
            if y % 2 == 1 {
                res = (res * x) % m;
            }
            x = (x * x) % m;
            y /= 2;
        }
        res
    }

    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }
        let n = nums.len();
        let m = 1_000_000_007;
        let mx = *nums.iter().max().unwrap() as i64;
        let mut v: BinaryHeap<Reverse<(i64, usize)>> = nums
            .into_iter()
            .enumerate()
            .map(|(i, num)| Reverse((num as i64, i)))
            .collect();

        // 先通过小根堆，将最小的值不断乘以 multiplier，直到大于等于 mx 或者 k 为 0
        // 那么此时，v 中的值都是大于等于 mx 的
        let mut k = k as i64;
        while let Some(Reverse((val, _))) = v.peek() {
            if *val >= mx || k == 0 {
                break;
            }
            let Reverse((mut min_val, idx)) = v.pop().unwrap();
            min_val *= multiplier as i64;
            v.push(Reverse((min_val, idx)));
            k -= 1;
        }

        let mut result = vec![0; n];
        let mut vec_v = v.into_vec();
        vec_v.sort_unstable_by_key(|Reverse((val, idx))| (*val, *idx));

        // 整体看成若干个周期，每个周期的长度为 n, 如果 i < k % n, 那么这个值的倍数需要加 1
        for (i, Reverse((val, idx))) in vec_v.iter().enumerate() {
            let t = k / n as i64 + if (i as i64) < k % n as i64 { 1 } else { 0 };
            result[*idx] = ((val % m) * Solution::quick_mul(multiplier as i64, t, m) % m) as i32;
        }
        result
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn get_final_state() {
        let nums = vec![4, 1, 2, 3];
        let k = 5;
        let multiplier = 2;
        let result = super::Solution::get_final_state(nums, k, multiplier);
        println!("{:?}", result);
    }
}
