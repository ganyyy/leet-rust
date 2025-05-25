fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    let mut graph = vec![vec![]; colors.len()];
    for edge in edges {
        let (u, v) = (edge[0], edge[1]);
        if u == v {
            return -1; // Self-loop detected
        }
        graph[u as usize].push(v);
    }

    println!("Graph: {:?}", graph);

    let mut memo = vec![None; colors.len()];

    fn dfs(
        node: usize,
        graph: &Vec<Vec<i32>>,
        colors: &Vec<u8>,
        memo: &mut Vec<Option<Vec<i32>>>,
    ) -> bool {
        if let Some(res) = &memo[node] {
            return !res.is_empty(); // Return true if already computed
        }
        memo[node] = Some(vec![]); // Mark as visited but not computed yet
        let mut count = vec![0; 26];
        for &nxt in &graph[node] {
            let valid = dfs(nxt as usize, graph, colors, memo);
            if !valid {
                return false; // Cycle detected
            }
            let nxt_cnt = memo[nxt as usize].as_ref().unwrap();
            for (i, &c) in nxt_cnt.iter().enumerate() {
                count[i] = count[i].max(c);
            }
        }
        println!("Node: {}, Count: {:?}", node, count);
        count[colors[node] as usize - b'a' as usize] += 1; // Increment the count for the current node's color
        memo[node] = Some(count.clone());
        true
    }

    let color = colors.as_bytes().to_vec();
    let mut max_value = 0;
    for (x, ch) in color.iter().enumerate() {
        let valid = dfs(x, &graph, &color, &mut memo);
        if !valid {
            return -1; // Cycle detected
        }
        let cnt = memo[x].as_ref().unwrap();
        max_value = max_value.max(cnt[(ch - b'a') as usize]);
    }

    max_value
}



mod tests {
    use super::*;

    #[test]
    fn test_largest_path_value() {
        let colors = "abaca".to_string();
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(largest_path_value(colors, edges), 3);
    }
}