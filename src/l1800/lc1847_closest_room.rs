#[allow(unused)]
fn closest_room(mut rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::{collections::BTreeSet, i32};

    // sort by room size. from large to small
    rooms.sort_unstable_by_key(|room| -room[1]);

    let query_len = queries.len();
    let mut query_ids = (0..query_len).collect::<Vec<_>>();

    // sort by query size. from large to small
    query_ids.sort_unstable_by_key(|&id| -queries[id][1]);

    let mut ans = vec![-1; query_len];
    let mut room_ids = BTreeSet::new();
    let mut room_idx = 0;

    for id in query_ids {
        let preferred_id = queries[id][0];
        let min_require_size = queries[id][1];

        while room_idx < rooms.len() && rooms[room_idx][1] >= min_require_size {
            room_ids.insert(rooms[room_idx][0]);
            room_idx += 1;
        }

        let mut diff = i32::MAX;
        // < preferred_id, next_back 获取小于 preferred_id 的最大值
        if let Some(&prev_idx) = room_ids.range(..preferred_id).next_back() {
            diff = (preferred_id - prev_idx).abs();
            ans[id] = prev_idx;
        }

        // >= preferred_id, next 获取大于等于 preferred_id 的最小值
        if let Some(&next_idx) = room_ids.range(preferred_id..).next() {
            if next_idx - preferred_id < diff {
                ans[id] = next_idx;
            }
        }
    }
    ans
}
