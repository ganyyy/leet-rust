use once_cell::sync::Lazy;

static GLOBAL_DROP_DP: Lazy<[i32; 1001]> = Lazy::new(|| {
    let mut drop_dp = [0; 1001];

    (1..=1000i32).for_each(|i| {
        drop_dp[i as usize] = i32::MAX;
        (1..=i).for_each(|j| {
            drop_dp[i as usize] = drop_dp[i as usize].min(j.max(1 + drop_dp[(i - j) as usize]));
        });
    });

    drop_dp
});

#[allow(unused)]
pub fn two_egg_drop_dp(n: i32) -> i32 {
    assert_eq!(n > 0 && n <= 1000, true);
    GLOBAL_DROP_DP
        .get(n as usize)
        .map(|&x| x)
        .unwrap_or_default()
}

#[allow(unused)]
pub fn two_egg_drop_math(mut n: i32) -> i32 {
    (1..)
        .scan(n, |state, cur| {
            state
                .gt(&&mut 0)
                .then(|| {
                    *state -= cur;
                    cur
                })
                .or_else(|| None)
        })
        .count() as i32
}
