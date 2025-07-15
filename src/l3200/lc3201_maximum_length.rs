pub fn maximum_length(nums: Vec<i32>) -> i32 {
    const K: usize = 2;

    let mut dp = vec![0; K];
    let mut ret = 0;
    for i in 0..K {
        // 枚举模数
        dp.fill(0);
        for &num in &nums {
            let num = (num % (K as i32)) as usize;
            // 注意: 这里计算的是前后两项的和mod k 一致的子序列
            // 所以这里要计算的是 dp[(i + K - num) % K] + 1
            dp[num] = dp[(i + K - num) % K] + 1;
            ret = ret.max(dp[num]);
        }
    }

    ret
}
