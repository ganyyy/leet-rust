pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut dp = vec![0; n as usize];
    dp[0] = 1;
    dp[1] = MOD - 1;
    let mut total = 0;
    let delay = delay as usize;
    let forget = forget as usize;
    let mut cnt_b = 0;
    for i in 0..n as usize {
        total = (total + dp[i]) % MOD;
        if i + delay >= n as usize {
            // 为啥是total呢? 好好想想
            cnt_b = (cnt_b + total) % MOD;
            continue;
        }
        dp[i + delay] = (dp[i + delay] + total) % MOD;
        if i + forget < n as usize {
            dp[i + forget] = (dp[i + forget] - total + MOD) % MOD;
        }
    }
    ((total + cnt_b) % MOD) as i32
}
