pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    let max = nums.iter().max().unwrap();

    let mut seen = vec![false; (*max + 1) as usize];
    let mut left = 0;
    let mut ret = 0;
    let mut sum = 0;

    for &x in &nums {
        while seen[x as usize] {
            seen[nums[left] as usize] = false;
            sum -= nums[left];
            left += 1;
        }
        seen[x as usize] = true;
        sum += x;
        ret = ret.max(sum);
    }

    ret
}
