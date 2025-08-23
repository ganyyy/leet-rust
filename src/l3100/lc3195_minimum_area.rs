pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    assert!(grid.len() > 0);
    assert!(grid[0].len() > 0);

    let (mut min_x, mut max_x) = (grid.len() as i32, -1);
    let (mut min_y, mut max_y) = (grid[0].len() as i32, -1);

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == 1 {
                min_x = min_x.min(x as i32);
                max_x = max_x.max(x as i32);
                min_y = min_y.min(y as i32);
                max_y = max_y.max(y as i32);
            }
        }
    }

    (max_x - min_x + 1) * (max_y - min_y + 1)
}
