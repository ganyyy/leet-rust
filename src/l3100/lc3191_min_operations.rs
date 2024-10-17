#[allow(unused)]
pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let length = nums.len();

    (0..length - 2).for_each(|i| {
        if nums[i] == 0 {
            nums[i + 1] = 1 ^ nums[i + 1];
            nums[i + 2] = 1 ^ nums[i + 2];
            ret += 1;
        }
    });

    if nums[length - 2] == 0 || nums[length - 1] == 0 {
        -1
    } else {
        ret
    }
}

#[allow(unused)]
pub fn min_operations2(nums: Vec<i32>) -> i32 {
    match nums.into_iter().fold((0, 1, 1), |(acc, x, y), z| {
        (acc + 1 - x, (y == x) as _, (z == x) as _)
    }) {
        (acc, 1, 1) => acc,
        _ => -1,
    }
}
