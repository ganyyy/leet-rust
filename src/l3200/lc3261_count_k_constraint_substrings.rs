#[allow(unused)]
pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = s.len();
    let s: Vec<u8> = s.bytes().collect::<Vec<u8>>(); // Convert to bytes for faster access
    let mut count = [0, 0];
    let mut right: Vec<usize> = vec![n; n];
    let mut prefix: Vec<i64> = vec![0; n + 1];
    let mut i = 0;

    for j in 0..n {
        count[(s[j] & 1) as usize] += 1; // Direct byte access
        while count[0] > k && count[1] > k {
            count[(s[i] & 1) as usize] -= 1;
            right[i] = j; // right[i]: 以i开头的最长子串的右边界
            i += 1;
        }
        prefix[j + 1] = prefix[j] + (j - i + 1) as i64;
    }

    let mut res: Vec<i64> = Vec::with_capacity(queries.len()); // Pre-allocate for efficiency
    for query in queries {
        let l = query[0] as usize;
        let r = query[1] as usize;
        let i = right[l].min(r + 1);
        let part1 = (i - l + 1) * (i - l) / 2;
        let part2 = prefix[r + 1] - prefix[i];
        res.push(part1 as i64 + part2);
    }
    res
}
