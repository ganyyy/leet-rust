#[allow(unused)]
fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let length = colors.len();
    if length < k as usize {
        return 0;
    }
    let mut cnt = 1;
    let mut ret = 0;
    (1..length - 1 + k as usize).for_each(|i| {
        if colors[i % length] != colors[(i - 1) % length] {
            cnt += 1;
        } else {
            cnt = 1;
        }
        if cnt >= k {
            ret += 1;
        }
    });
    ret
}
