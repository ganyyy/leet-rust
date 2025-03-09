use std::i32;

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut ret = i32::MAX;

    let mut left = 0usize;
    let mut sum = 0i32;
    let mut buffer = [0i32; 32];

    fn add(buffer: &mut [i32; 32], value: i32) -> i32 {
        let mut total = 0;
        (0..buffer.len()).for_each(|idx| {
            if value & (1 << idx) != 0 {
                buffer[idx] += 1;
            }
            if buffer[idx] > 0 {
                total |= 1 << idx;
            }
        });
        total
    }

    fn sub(buffer: &mut [i32; 32], value: i32) -> i32 {
        let mut total = 0;
        (0..buffer.len()).for_each(|idx| {
            if value & (1 << idx) != 0 {
                buffer[idx] -= 1;
            }
            if buffer[idx] > 0 {
                total |= 1 << idx;
            }
        });
        total
    }

    nums.iter().enumerate().for_each(|(right, &value)| {
        sum = add(&mut buffer, value);
        while sum >= k && left <= right {
            ret = ret.min((right - left + 1) as i32);
            sum = sub(&mut buffer, nums[left]);
            left += 1;
        }
    });

    if ret == i32::MAX { -1 } else { ret }
}

pub fn minimum_subarray_length2(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = usize::MAX;
    let mut left = 0;
    let mut bottom = 0;
    let mut right_or = 0;
    for right in 0..nums.len() {
        right_or |= nums[right];
        while left <= right && (nums[left] | right_or) >= k {
            ans = ans.min(right - left + 1);
            left += 1;
            if bottom < left {
                // 重新构建一个栈
                for i in (left..right).rev() {
                    nums[i] |= nums[i + 1];
                }
                bottom = right;
                right_or = 0;
            }
        }
    }
    if ans == usize::MAX { -1 } else { ans as _ }
}
