#[allow(unused)]
struct Solution;

impl Solution {
    #[allow(unused)]
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let length = nums.len();

        let mut ret = vec![-1i32; length - k as usize + 1];
        let mut cnt = 1;

        (1..length).for_each(|right| {
            if nums[right] - 1 == nums[right - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }
            if cnt >= k {
                ret[right - k as usize + 1] = nums[right];
            }
        });

        ret
    }
}
