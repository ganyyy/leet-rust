#[allow(unused)]
pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    let length = nums.len();
    let mut ret = nums[length - 1] - nums[0];
    (1..length).for_each(|i| {
        // 0..i +k
        // i..length -k
        let max = (nums[i - 1] + k).max(nums[length - 1] - k);
        let min = (nums[0] + k).min(nums[i] - k);
        ret = ret.min(max - min);
    });
    ret
}
