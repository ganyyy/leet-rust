fn longest_cycle(edges: Vec<i32>) -> i32 {
    let length = edges.len();
    let mut ret = -1;
    let mut cur_time = 1;
    let mut visited = vec![0i32; length];

    for i in 0i32..length as i32 {
        let mut x = i;
        let start_time = cur_time; // 从x出发的时间

        while x != -1 && visited[x as usize] == 0 {
            // x第一次访问
            visited[x as usize] = cur_time; // 记录首次访问时间
            cur_time += 1; // 时间+1
            x = edges[x as usize]; // 访问下一个节点
        }
        // 如果 x == -1, 说明并没有形成环
        if x != -1 && visited[x as usize] >= start_time {
            ret = ret.max(cur_time - visited[x as usize]);
        }
    }

    ret
}
