fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;

    queries.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut diff = vec![0; nums.len() + 1];
    let mut sum = 0;
    let mut query_index = 0;

    for (i, &v) in nums.iter().enumerate() {
        sum += diff[i];

        while query_index < queries.len() && queries[query_index][0] <= i as i32 {
            heap.push(queries[query_index][1]);
            query_index += 1;
        }

        while sum < v && heap.peek().is_some_and(|&v| v >= i as i32) {
            let right = heap.pop().unwrap();
            sum += 1;
            diff[right as usize + 1] -= 1;
        }

        if sum < v {
            return -1;
        }
    }

    heap.len() as i32
}
