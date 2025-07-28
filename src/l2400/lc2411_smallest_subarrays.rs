fn smallest_subarrays(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0; n];

    for i in 0..n {
        result[i] = 1;
        let val = nums[i];
        if i == 0 {
            continue;
        }
        for j in (0..i).rev() {
            if nums[j] | val == nums[j] {
                break;
            }
            nums[j] |= val;
            result[j] = (i - j) as i32 + 1;
        }
    }
    result
}
