pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
    let low = start.to_string();
    let high = finish.to_string();
    let length = high.len();
    let low = format!("{:0>length$}", low);
    let diff = length - s.len(); // 前缀可以补充的数字位数

    fn dfs(
        low: &[u8],
        high: &[u8],
        s: &[u8],
        idx: usize,
        limit_low: bool,
        limit_high: bool,
        limit: u8,
        diff: usize,
        memo: &mut Vec<i64>,
    ) -> i64 {
        if idx == high.len() {
            return 1; // 这是一个有效方案
        }
        let mut ret = 0i64;
        if !limit_low && !limit_high {
            if memo[idx] != 0 {
                return memo[idx];
            }
        }
        let mut lo = 0;
        if limit_low {
            lo = low[idx] - b'0';
        }
        let mut hi = 9;
        if limit_high {
            hi = high[idx] - b'0';
        }

        if idx < diff {
            (lo..=limit.min(hi)).for_each(|b| {
                ret += dfs(
                    low,
                    high,
                    s,
                    idx + 1,
                    limit_low && b == lo,
                    limit_high && b == hi,
                    limit,
                    diff,
                    memo,
                )
            });
        } else {
            let x = s[idx - diff] - b'0';
            if lo <= x && x <= hi.min(limit) {
                ret += dfs(
                    low,
                    high,
                    s,
                    idx + 1,
                    limit_low && x == lo,
                    limit_high && x == hi,
                    limit,
                    diff,
                    memo,
                );
            }
        }
        if !limit_low && !limit_high {
            memo[idx] = ret;
        }
        ret
    }

    let mut memo = vec![0i64; length];
    dfs(
        low.as_bytes(),
        high.as_bytes(),
        s.as_bytes(),
        0,
        true,
        true,
        limit as u8,
        diff,
        &mut memo,
    )
}
