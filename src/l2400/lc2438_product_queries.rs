pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    const MOD: i64 = 1_000_000_007;
    let mut n = n;
    let mut powers = vec![];
    while n > 0 {
        let low_bit = n & -n;
        powers.push(low_bit);
        n ^= low_bit;
    }

    let mut results = Vec::with_capacity(queries.len());

    for query in queries {
        let (start, end) = (query[0] as usize, query[1] as usize);
        let mut multi: i64 = 1;
        for x in start..=end {
            multi = (multi * powers[x] as i64) % MOD;
        }
        results.push(multi as i32);
    }

    results
}
