use std::sync::LazyLock;

static CACHE: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| {
    const MOD: i32 = 1e9 as i32 + 7;
    let mut ret = vec![vec![0; 4]; 5000];

    ret[0] = vec![1, 1, 1, 1];

    /*
       1(A) --- 8(B) --- 3(A)
        |                 |
       6(C) --- 0(D) --- 4(C)
        |                 |
       7(A) --- 2(B) --- 9(A)


       A = B' + C'
       B = 2 * A'
       C = 2 * A' + D'
       D = 2 * C'
    */

    (1..ret.len()).for_each(|i: usize| {
        ret[i][0] = (ret[i - 1][1] + ret[i - 1][2]) % MOD;
        ret[i][1] = ret[i - 1][0] * 2 % MOD;
        ret[i][2] = (ret[i - 1][0] * 2 + ret[i - 1][3]) % MOD;
        ret[i][3] = ret[i - 1][2] * 2 % MOD;
    });

    ret
});

#[allow(unused)]
pub fn knight_dialer(n: i32) -> i32 {
    if n == 1 {
        return 10;
    }
    let last = CACHE[(n - 1) as usize].clone();

    last[0] * 4 + last[1] * 2 + last[2] * 2 + last[3]
}
