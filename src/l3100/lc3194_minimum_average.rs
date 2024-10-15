#[allow(unused)]
fn minimum_average(mut nums: Vec<i32>) -> f64 {
    nums.sort();

    nums.iter()
        .zip(nums.iter().rev())
        .take(nums.len() / 2)
        .map(|(&a, &b)| (a + b) as f64 / 2.0)
        .fold(f64::MAX, f64::min) // fold: 有初始值的reduce
}
