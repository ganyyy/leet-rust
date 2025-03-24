pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut ret = vec![vec![0; n as usize]; m as usize];

    let path_count = (m + n - 1) as i32;

    // 从右上到左下迭代对角线

    // 比如 [3, 3] 的矩阵的迭代顺序如下
    // [min_j, max_j]
    // [2,2]
    // [1,2]
    // [0,2]
    // [0,1]
    // [0,0]
    (1..=path_count).for_each(|k| {
        let min_j = 0.max(n as i32 - k as i32);
        let max_j = (n - 1).min(path_count - k);

        let mut set = 0usize;
        (min_j..=max_j).for_each(|j| {
            let i = k + j - n;
            ret[i as usize][j as usize] = set.count_ones() as i32;
            set |= 1 << grid[i as usize][j as usize];
        });
        set = 0;
        (min_j..=max_j).rev().for_each(|j| {
            let i = k + j - n;
            println!("{i},{j}");
            ret[i as usize][j as usize] =
                (ret[i as usize][j as usize] - set.count_ones() as i32).abs();
            set |= 1 << grid[i as usize][j as usize];
        });
    });

    ret
}
