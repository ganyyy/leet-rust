pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    assert_eq!(basket1.len(), basket2.len());

    use std::collections::HashMap;
    let mut count: HashMap<i32, i32> = HashMap::new();

    for (i, v) in basket1.iter().enumerate() {
        *count.entry(*v).or_insert(0) += 1;
        *count.entry(basket2[i]).or_insert(0) -= 1;
    }

    let mut queue = vec![];
    let mut mn = i32::MAX;
    for (k, cnt) in count {
        if cnt % 2 != 0 {
            return -1;
        }
        mn = mn.min(k);
        for _ in 0..(cnt.abs() / 2) {
            queue.push(k);
        }
    }

    queue.sort_unstable();

    let mut ret = 0 as i64;

    for i in 0..(queue.len() / 2) {
        ret += queue[i].min(mn * 2) as i64;
    }

    ret
}
