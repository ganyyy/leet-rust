use std::{collections::BTreeMap, i32};

struct MyCalendar {
    tree: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        let mut tree = BTreeMap::new();
        tree.insert(i32::MAX, 0);
        MyCalendar { tree }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        self.tree
            .range(..end)
            .next_back()
            .map_or(true, |(&_, &prev_end)| prev_end <= start)
            .then(|| self.tree.insert(start, end))
            .is_some()
    }
}
