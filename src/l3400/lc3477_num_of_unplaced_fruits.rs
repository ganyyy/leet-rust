struct SegTree(Vec<i32>);

impl SegTree {
    pub fn new(arr: Vec<i32>) -> Self {
        assert!(!arr.is_empty(), "Array must not be empty");
        let length = arr.len();
        let tree = vec![0; length.next_power_of_two() as usize];
        let mut seg_tree = SegTree(tree);
        seg_tree.build(&arr, 1, 0, length - 1);
        seg_tree
    }

    fn build(&mut self, arr: &[i32], curr: usize, left: usize, right: usize) {
        if left == right {
            self.0[curr] = arr[left];
        } else {
            let mid = (left + right) / 2;
            self.build(arr, 2 * curr, left, mid);
            self.build(arr, 2 * curr + 1, mid + 1, right);
            self.maintain(curr);
        }
    }

    fn maintain(&mut self, curr: usize) {
        self.0[curr] = self.0[2 * curr].max(self.0[2 * curr + 1]);
    }

    pub fn find_first_and_update(
        &mut self,
        curr: usize,
        left: usize,
        right: usize,
        target: i32,
    ) -> Option<usize> {
        if self.0[curr] < target {
            return None;
        }
        if left == right {
            self.0[curr] = -1;
            return Some(left);
        }
        let mid = (left + right) / 2;
        if let Some(index) = self.find_first_and_update(curr << 1, left, mid, target) {
            return Some(index);
        }
        let ret = self.find_first_and_update(curr << 1 | 1, mid + 1, right, target);
        self.maintain(curr);
        ret
    }

    fn next(curr: usize) -> (usize, usize) {
        (curr << 1, curr << 1 | 1)
    }
}

struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let size = baskets.len() - 1;
        let mut seg = SegTree::new(baskets);
        let mut ret = 0;
        for fruit in fruits {
            seg.find_first_and_update(1, 0, size, fruit).or_else(|| {
                ret += 1;
                None
            });
        }
        ret
    }
}
