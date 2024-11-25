#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut count = vec![0; 2];
        let mut left = 0usize;
        let mut ret = 0;
        let bs = s.as_bytes();
        bs.into_iter().enumerate().for_each(|(right, c)| {
            count[(c & 1) as usize] += 1;
            while count[0] > k && count[1] > k {
                count[(bs[left] & 1) as usize] -= 1;
                left += 1;
            }

            ret += right - left + 1;
        });

        ret as i32
    }
}
