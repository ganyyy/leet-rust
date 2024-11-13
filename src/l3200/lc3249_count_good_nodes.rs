#[allow(unused)]
pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
    let mut path: Vec<Vec<i32>> = vec![vec![]; edges.len() + 1];
    edges.iter().for_each(|edge| {
        let (p1, p2) = (edge[0], edge[1]);
        path[p1 as usize].push(p2);
        path[p2 as usize].push(p1);
    });

    let mut ret = 0;

    fn dfs(current: i32, father: i32, path: &Vec<Vec<i32>>, ret: &mut i32) -> i32 {
        let mut good = true;
        let mut first = -1;
        let mut size = 1;

        path[current as usize].iter().for_each(|&next| {
            if next == father {
                return;
            }

            let sub_size = dfs(next, current, path, ret);
            size += sub_size;
            if first == -1 {
                first = sub_size;
            } else {
                good = good && sub_size == first;
            }
        });

        if good {
            *ret += 1;
        }

        size
    }

    dfs(0, -1, &path, &mut ret);

    ret
}
