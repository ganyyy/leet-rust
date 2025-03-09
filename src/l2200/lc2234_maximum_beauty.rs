pub fn maximum_beauty(
    mut flowers: Vec<i32>,
    new_flowers: i64,
    target: i32,
    full: i32,
    partial: i32,
) -> i64 {
    let (target, full, partial) = (target as i64, full as i64, partial as i64);
    let length = flowers.len() as i64;
    // 假设所有的flowers都是0, 那么需要的花朵数是target * length
    let mut left_flowers = new_flowers - (target as i64 * length);
    flowers.iter_mut().for_each(|flower| {
        *flower = (*flower).min(target as i32); // 超出部分无意义, 所以上限取target即可
        left_flowers += *flower as i64; // 将每个花园当前已有的花朵数返还给left_flowers
    });

    if left_flowers == new_flowers {
        // 所有的花园的花朵数量都是>=target的, 那么直接返回full * length即可
        return length * full;
    }
    if left_flowers >= 0 {
        // 允许将所有的花园都填充到>=target, 并且还有剩余的花朵数
        // 那么此时就需要考虑怎么才能让整体最大
        // 有两种情况可以获取到最大值:
        //     return length * full;
        //     return (length - 1) * full + partial * (target - 1);
        return ((target - 1) * partial + (length - 1) * full).max(length * full);
    }

    // 开始排序双指针
    // 将整体分为三部分:
    // [:j+1], [j+1:i], [i:]
    // [:j+1]表示花园需要进行填充, 并且填充后的最大值不会超过flower[j]
    // [i:]表示都是>=target的花园
    flowers.sort_unstable();
    let (mut ret, mut pre_sum, mut j) = (0, 0, 0);

    (1..length + 1).for_each(|i| {
        // 回收这个花园的花朵数, 因为在前边的计算中, 这个位置相当于是保留了一部分被消耗的花朵数
        left_flowers += target - flowers[(i - 1) as usize] as i64;
        if left_flowers < 0 {
            return;
        }

        // 看看当前的left_flowers能够填充多少个花园
        while j < i && flowers[j as usize] as i64 * j <= pre_sum + left_flowers {
            pre_sum += flowers[j as usize] as i64;
            j += 1;
        }

        // 前j个花园的最大平均值
        let avg = (left_flowers + pre_sum) / j;

        ret = ret.max(avg * partial + (length - i) * full);
    });

    ret
}

mod test {
    #[test]
    fn test_maximum_beauty() {}
}
