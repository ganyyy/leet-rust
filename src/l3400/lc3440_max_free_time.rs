pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    assert_eq!(start_time.len(), end_time.len());

    let length = start_time.len();
    let mut ret = 0;

    let sz = |idx: usize| -> i32 {
        if idx == 0 {
            return start_time[0];
        } else if idx == length {
            return event_time - end_time.last().unwrap();
        } else {
            return start_time[idx] - end_time[idx - 1];
        }
    };

    // 最大, 次大, 三大
    let (mut a, mut b, mut c) = (Some(0usize), None::<usize>, None::<usize>);

    for idx in 1..=length {
        let size = sz(idx);
        if a.is_none_or(|x| size > sz(x)) {
            (a, b, c) = (Some(idx), a, b);
        } else if b.is_none_or(|x| size > sz(x)) {
            (b, c) = (Some(idx), b);
        } else if c.is_none_or(|x| size > sz(x)) {
            c = Some(idx);
        }
    }

    for (idx, &end) in end_time.iter().enumerate() {
        let size = end - start_time[idx];

        if a.is_some_and(|a| a != idx && a != idx + 1 && size <= sz(a))
            || b.is_some_and(|b| b != idx && b != idx + 1 && size <= sz(b))
            || c.is_some_and(|c| size <= sz(c))
        // 假设 a = idx, b = idx + 1, 那么此时 c 一定和 idx 是不相邻的, 可以放进去
        {
            // 将 idx 的事件移动到a/b/c的空闲时间段. |__|idx|__|
            ret = ret.max(sz(idx) + size + sz(idx + 1));
        } else {
            // 将 idx 的事件移动到前后空间的开头或者结尾. |idx|__|__|/|__|__|idx|
            ret = ret.max(sz(idx) + sz(idx + 1));
        }
    }

    ret
}
