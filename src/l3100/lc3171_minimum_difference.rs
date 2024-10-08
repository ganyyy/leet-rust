#[allow(unused)]
pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = i32::MAX;
    (0..nums.len()).for_each(|i| {
        let num = nums[i];
        ans = ans.min((num - k).abs());

        for j in (0..i).rev() {
            if nums[j] | num == nums[j] {
                break;
            }
            nums[j] |= num;
            ans = ans.min((nums[j] - k).abs());
        }
    });
    ans
}
