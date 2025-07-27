fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let max = nums.iter().fold(0, |acc, &x| acc | x);
    let length = nums.len();
    let mut count = 0;
    for i in 0..(1 << length) {
        let mut subset = 0;
        for j in 0..length {
            if i & (1 << j) != 0 {
                subset |= nums[j];
            }
            if subset == max {
                count += 1;
                break; // No need to check further, we found a valid subset
            }
        }
    }
    count
}
