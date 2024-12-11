#[allow(unused)]
pub fn max_spending(mut values: Vec<Vec<i32>>) -> i64 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::with_capacity(values.len());

    fn add_item(idx: usize, value: &mut Vec<i32>, heap: &mut BinaryHeap<(Reverse<i32>, usize)>) {
        value.pop().map(|v| heap.push((Reverse(v), idx)));
    }

    values.iter_mut().enumerate().for_each(|(idx, value)| {
        add_item(idx, value, &mut heap);
    });

    let mut day = 1;
    let mut ret = 0i64;

    while let Some((Reverse(val), idx)) = heap.pop() {
        ret += val as i64 * day;
        day += 1;
        add_item(idx, &mut values[idx], &mut heap);
    }

    ret
}
