#[allow(unused)]
pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    let first_row = board[0].clone();
    let first_col = board.iter().map(|row| row[0]).collect::<Vec<i32>>();
    let (mut row_cnt, mut col_cnt) = (vec![0; 2], vec![0; 2]);

    first_row.iter().for_each(|&x| row_cnt[x as usize] += 1);
    first_col.iter().for_each(|&x| col_cnt[x as usize] += 1);

    if ((row_cnt[0] - row_cnt[1]) as i32).abs() > 1 || ((col_cnt[0] - col_cnt[1]) as i32).abs() > 1
    {
        return -1;
    }

    // check every row
    for i in 0..board.len() {
        let row = &board[i];
        let is_same = row[0] == first_row[0];
        for j in 0..row.len() {
            if (row[j] == first_row[j]) != is_same {
                return -1;
            }
        }
    }

    fn min_swap(row: &Vec<i32>, row_cnt: &[i32]) -> i32 {
        let n = row.len() as i32;
        let mut most_val = 0i32;
        if row_cnt[1] > row_cnt[0] {
            most_val = 1;
        }

        let mut diff = 0;
        row.iter().enumerate().for_each(|(i, &x)| {
            diff += (i % 2) as i32 ^ x ^ most_val;
        });

        if n % 2 == 0 {
            return diff.min(n - diff) / 2;
        }
        diff / 2
    }

    min_swap(&first_row, &row_cnt) + min_swap(&first_col, &col_cnt)
}
