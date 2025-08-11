pub fn number_of_ways(n: i32, x: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut dp = vec![0i64; n as usize + 1];
    dp[0] = 1;
    let mut i: i64 = 1;
    while i.pow(x as u32) <= n as i64 {
        let v = i.pow(x as u32);

        (v..=n as i64).rev().for_each(|s| {
            dp[s as usize] = (dp[s as usize] + dp[(s - v) as usize]) % MOD;
        });

        i += 1;
    }

    dp[n as usize] as i32 % MOD as i32
}
