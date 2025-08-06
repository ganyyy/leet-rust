pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
    let n = fruits.len();
    assert!(n > 1);
    assert_eq!(n, fruits[0].len());

    // 列多分配一个给边界处理
    let mut f = vec![vec![0; n + 1]; n - 1];

    fn dp(f: &mut Vec<Vec<i32>>, fruits: &Vec<Vec<i32>>) -> i32 {
        let n = fruits.len();
        f.iter_mut().for_each(|row| row.fill(i32::MIN));
        f[0][n - 1] = fruits[0][n - 1];

        for i in 1..n - 1 {
            for j in (i + 1).max(n - 1 - i)..n {
                let pre = &f[i - 1];
                f[i][j] = pre[j - 1].max(pre[j]).max(pre[j + 1]) + fruits[i][j]
            }
        }

        return f[n - 2][n - 1];
    }

    let mut ans = 0;
    // 对角线
    for (i, row) in fruits.iter().enumerate() {
        ans += row[i];
    }

    ans += dp(&mut f, &fruits);
    // 右下旋转到左上
    for i in 0..n {
        for j in 0..i {
            fruits[j][i] = fruits[i][j];
        }
    }

    ans + dp(&mut f, &fruits)
}
