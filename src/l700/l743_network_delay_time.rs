#[allow(unused)]
fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // (node, time)
    let mut heap = std::collections::BinaryHeap::new();
    let mut graph = vec![vec![]; n as usize];
    times.into_iter().for_each(|time| {
        let (from, to, t) = (time[0] as usize - 1, time[1] as usize - 1, time[2]);
        graph[from].push((to, t));
    });
    let mut dist = vec![std::i32::MAX; n as usize];
    dist[k as usize - 1] = 0;

    heap.push((k as usize - 1, 0));

    while heap.len() > 0 {
        let (node, time) = heap.pop().unwrap();
        if dist[node] < time {
            continue;
        }
        for (next, t) in graph[node].iter() {
            let next_time = time + t;
            if next_time < dist[*next] {
                dist[*next] = next_time;
                heap.push((*next, next_time));
            }
        }
    }

    let max_time = dist.into_iter().max().unwrap();
    if max_time == std::i32::MAX {
        -1
    } else {
        max_time
    }
}
