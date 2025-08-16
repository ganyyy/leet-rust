pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    if k <= 0 {
        return 1.0;
    }
    assert!(n >= k);

    let mut dp = vec![0.0; (k + max_pts) as usize];

    (k..=(n.min(k + max_pts - 1))).for_each(|k| {
        dp[k as usize] = 1.0;
    });

    dp[k as usize - 1] = (max_pts.min(n - k + 1) as f64) / (max_pts as f64);

    (0..=k - 2).rev().for_each(|k| {
        let k = k as usize;
        dp[k] = dp[k + 1] - (dp[k + (max_pts + 1) as usize] - dp[k + 1]) / (max_pts as f64);
    });

    dp[0]
}
