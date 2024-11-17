#[allow(unused)]
fn num_friend_requests(ages: Vec<i32>) -> i32 {
    const LENGTH: usize = 121;
    let mut group = vec![0; LENGTH]; // 到一定区间的前缀和

    ages.into_iter().for_each(|age| group[age as usize] += 1);
    (1..LENGTH).for_each(|age| group[age] += group[age - 1]);

    let mut total = 0;

    fn check(x: usize, y: usize) -> bool {
        if y > x {
            return false;
        }
        if y <= x / 2 + 7 {
            return false;
        }
        if y > 100 && x < 100 {
            return false;
        }

        true
    }

    let mut x = 1;
    (1..LENGTH).for_each(|y| {
        let y_count = group[y] - group[y - 1];
        if y_count == 0 {
            return;
        }
        if x < y {
            x = y;
        }
        while x < LENGTH && check(x, y) {
            x += 1;
        }
        let x_count = group[x - 1] - group[y - 1] - 1;
        if x_count <= 0 {
            return;
        }
        total += y_count * x_count;
    });

    total
}
