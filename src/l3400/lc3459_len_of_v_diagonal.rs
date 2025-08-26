const DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    assert!(grid.len() > 0 && grid[0].len() > 0);

    let mut memo = vec![vec![vec![vec![0; 2]; 4]; grid[0].len()]; grid.len()];

    fn dfs(
        memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
        grid: &Vec<Vec<i32>>,
        mut i: i32,
        mut j: i32,
        dir: usize,
        can_turn: bool,
        target: i32,
    ) -> i32 {
        i += DIRS[dir].0;
        j += DIRS[dir].1;
        if i < 0 || i >= memo.len() as i32 || j < 0 || j >= memo[0].len() as i32 {
            return 0;
        }
        if grid[i as usize][j as usize] != target {
            return 0;
        }

        let mut res = dfs(memo, grid, i, j, dir, can_turn, 2 - target);
        if can_turn {
            res = res.max(dfs(memo, grid, i, j, (dir + 1) % 4, false, 2 - target));
        }

        memo[i as usize][j as usize][dir][can_turn as usize] = res + 1;
        res + 1
    }

    let mut ans = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v != 1 {
                continue;
            }

            for dir in 0..4 {
                ans = ans.max(dfs(&mut memo, &grid, i as i32, j as i32, dir, true, 2) + 1);
            }
        }
    }
    ans
}
