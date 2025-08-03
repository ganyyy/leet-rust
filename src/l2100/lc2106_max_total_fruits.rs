pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
    let mut left = fruits
        .binary_search_by_key(&(start_pos - k), |v| v[0])
        .unwrap_or_else(|x| x);

    let mut pre_sum = 0;

    let mut right = left;

    while right < fruits.len() && fruits[right][0] <= start_pos {
        pre_sum += fruits[right][1];
        right += 1;
    }

    let mut ret = pre_sum;

    while right < fruits.len() && fruits[right][0] - start_pos <= k {
        pre_sum += fruits[right][1];

        // 先向右再向左: (right-start_pos) + (right-left) <= k
        // 先向左再向右: (start_pos-left) + (right-left) <= k
        while fruits[right][0] * 2 - fruits[left][0] - start_pos > k
            && fruits[right][0] - fruits[left][0] * 2 + start_pos > k
        {
            pre_sum -= fruits[left][1];
            left += 1;
        }

        ret = ret.max(pre_sum);
        right += 1;
    }

    ret
}
