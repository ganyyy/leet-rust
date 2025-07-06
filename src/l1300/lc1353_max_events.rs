pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mx = events.iter().fold(0, |max, event| event[1].max(max));

    let mut groups = vec![vec![]; (mx + 1) as usize];

    events.iter().for_each(|event| {
        groups[event[0] as usize].push(event[1]);
    });

    let mut heap = BinaryHeap::new();
    let mut total = 0;
    groups.iter().enumerate().for_each(|(day, events)| {
        while let Some(&Reverse(end)) = heap.peek() {
            if end < day as i32 {
                heap.pop();
            } else {
                break;
            }
        }
        events.iter().for_each(|&end| {
            heap.push(Reverse(end));
        });
        if let Some(_) = heap.pop() {
            total += 1;
        }
    });

    total
}
