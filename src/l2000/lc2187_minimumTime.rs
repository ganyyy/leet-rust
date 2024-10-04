#[allow(unused)]
pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut left = 1;
    let mut right = *time.iter().min().unwrap() as i64 * total_trips as i64;

    while left < right {
        let mid = left + (right - left) / 2;
        let total_cost = time
            .iter()
            .filter(|&&t| t as i64 <= mid)
            .map(|&t| mid / t as i64)
            .sum::<i64>();
        if total_cost < total_trips as i64 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
