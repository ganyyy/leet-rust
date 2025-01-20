pub fn max_value_of_coins(mut piles: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp = vec![0; (k + 1) as usize];
    let mut sum_length = 0;

    piles.iter_mut().for_each(|pile| {
        // pile的前缀和
        let mut sum = 0;
        let current_length = pile.len() as i32;
        pile.iter_mut().for_each(|coin| {
            sum += *coin;
            *coin = sum;
        });

        // 可选的最大长度
        sum_length = (sum_length + current_length).min(k);

        // 因为后态只与前态有关，所以可以从后往前遍历
        (1..=sum_length).rev().for_each(|idx| {
            // 从当前pile中选取指定数量的硬币
            pile[..idx.min(current_length) as usize]
                .iter()
                .enumerate()
                .for_each(|(weight, &val)| {
                    dp[idx as usize] =
                        dp[idx as usize].max(dp[(idx - weight as i32 - 1) as usize] + val)
                });
        });
    });

    dp[k as usize]
}
