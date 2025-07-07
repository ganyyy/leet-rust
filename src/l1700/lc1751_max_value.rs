pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    // 按照结束时间排序
    events.sort_by_key(|x| x[1]);

    let length = events.len();

    // dp[i][j] 表示前 i 个事件中选择 j 个事件的最大值
    let mut dp = vec![vec![0; k as usize + 1]; length + 1];

    for (i, event) in events.iter().enumerate() {
        let (start, _, value) = (event[0], event[1], event[2]);

        // 找到首个不满足条件的事件的位置
        let pos = events[..i].partition_point(|a| a[1] < start);
        for j in 1..=k as usize {
            // 选择或者不选择当前事件
            dp[i + 1][j] = dp[i][j].max(dp[pos][j - 1] + value);
        }
    }

    dp[length][k as usize]
}
