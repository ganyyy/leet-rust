fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    if length == 0 {
        return vec![];
    }
    nums.sort();
    const DEFAULT_PREV: usize = usize::MAX;
    let mut dp = vec![1; length];
    let mut prev = vec![DEFAULT_PREV; length];
    let mut max_index = 0usize;

    nums.iter().enumerate().for_each(|(i, &num)| {
        for j in 0..i {
            if num % nums[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = j;
            }
        }
        if dp[i] > dp[max_index] {
            max_index = i;
        }
    });

    let mut ret = Vec::with_capacity(dp[max_index]);
    ret.push(nums[max_index]);
    loop {
        let prev_index = prev[max_index];
        if prev_index == DEFAULT_PREV {
            break;
        }
        ret.push(nums[prev_index]);
        max_index = prev_index;
    }

    ret
}
