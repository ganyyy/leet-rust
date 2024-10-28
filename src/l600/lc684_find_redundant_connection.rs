#[allow(unused)]
fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let length = edges.len();
    let mut parent: Vec<i32> = vec![0; length + 1];
    (1..=length).for_each(|i| parent[i] = i as i32);

    let mut find = |parent: &mut Vec<i32>, mut x: i32| {
        while x != parent[x as usize] {
            parent[x as usize] = parent[parent[x as usize] as usize];
            x = parent[x as usize];
        }
        x
    };

    let mut merge = |x: i32, y: i32| {
        let root_x = find(&mut parent, x);
        let root_y = find(&mut parent, y);
        if root_x != root_y {
            parent[root_x as usize] = root_y;
            false
        } else {
            true
        }
    };

    edges
        .into_iter()
        .find(|edge| merge(edge[0], edge[1]))
        .unwrap_or_else(|| vec![])
}
