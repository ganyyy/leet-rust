use std::{collections::HashMap, i32};

struct Pair {
    first: i32,
    second: i32,
}

struct MyCalendarTwo {
    map: HashMap<i32, Pair>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            map: HashMap::new(),
        }
    }

    fn update(&mut self, start: i32, end: i32, val: i32, left: i32, right: i32, idx: i32) {
        if left > end || right < start {
            return;
        }
        if start <= left && right <= end {
            self.map
                .entry(idx)
                .and_modify(|e| {
                    e.first += val;
                    e.second += val;
                })
                .or_insert_with(|| Pair {
                    first: val,
                    second: val,
                });
            return;
        }
        let mid = (left + right) / 2;
        self.update(start, end, val, left, mid, 2 * idx);
        self.update(start, end, val, mid + 1, right, 2 * idx + 1);

        let left_child_first = self.map.get(&(2 * idx)).map_or(0, |e| e.first);
        let right_child_first = self.map.get(&(2 * idx + 1)).map_or(0, |e| e.first);
        self.map
            .entry(idx)
            .and_modify(|p| {
                p.first = p.second + left_child_first.max(right_child_first);
            })
            .or_insert_with(|| Pair {
                first: left_child_first.max(right_child_first),
                second: 0,
            });
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        self.update(start, end - 1, 1, 0, 1e9 as i32 - 1, 1);
        (self.map.get(&1).unwrap().first > 2)
            .then(|| {
                self.update(start, end - 1, -1, 0, 1e9 as i32 - 1, 1);
                false
            })
            .unwrap_or(true)
    }
}
