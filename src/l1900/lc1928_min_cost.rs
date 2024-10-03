pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
    let node_count = passing_fees.len();
    let max_time = max_time as usize;

    // 初始化 dp 数组
    let mut dp = vec![vec![std::i32::MAX; node_count]; max_time + 1];
    dp[0][0] = passing_fees[0];

    // 更新 dp 数组
    (1..=max_time).for_each(|time| {
        edges.iter().for_each(|edge| {
            let (start, end, cost) = (edge[0] as usize, edge[1] as usize, edge[2] as usize);
            if cost <= time {
                if dp[time - cost][start] != std::i32::MAX {
                    dp[time][end] = dp[time][end].min(dp[time - cost][start] + passing_fees[end]);
                }
                if dp[time - cost][end] != std::i32::MAX {
                    dp[time][start] =
                        dp[time][start].min(dp[time - cost][end] + passing_fees[start]);
                }
            }
        });
    });

    // 计算最小费用
    dp.iter()
        .map(|costs| *costs.last().unwrap())
        .min()
        .filter(|&min_cost| min_cost != std::i32::MAX)
        .unwrap_or(-1)
}
