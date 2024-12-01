#[derive(Default)]
struct Bits(i32);

impl Bits {
    pub fn new() -> Self {
        Bits(0)
    }

    pub fn set(&mut self, i: i32) {
        self.0 |= 1 << i;
    }

    pub fn is_set(&self, i: i32) -> bool {
        self.0 & (1 << i) != 0
    }

    pub fn unset(&mut self, i: i32) {
        self.0 &= !(1 << i);
    }
}

#[derive(Default)]
struct Data {
    cols: Bits,
    ml: Bits,
    sl: Bits,
}

pub fn total_n_queens(n: i32) -> i32 {
    let mut data = Data::default();
    let mut res = 0;

    fn dfs(row: i32, total: i32, res: &mut i32, data: &mut Data) {
        if row == total {
            *res += 1;
            return;
        }
        (0..total).for_each(|col| {
            let (c, m, s) = (col, row - col + total, row + col);
            if !data.cols.is_set(c) && !data.ml.is_set(m) && !data.sl.is_set(s) {
                data.cols.set(c);
                data.ml.set(m);
                data.sl.set(s);
                dfs(row + 1, total, res, data);
                data.cols.unset(c);
                data.ml.unset(m);
                data.sl.unset(s);
            }
        });
    }

    dfs(0, n, &mut res, &mut data);

    res
}
