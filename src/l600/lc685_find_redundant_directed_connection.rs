struct Solution;

impl Solution {
    #[allow(unused)]
    fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::vec;
        let n = edges.len();
        let mut nf = UnionFind::new(n + 1);
        let mut parent = (0i32..(n + 1) as i32).collect::<Vec<i32>>();

        let (mut conflict_edge, mut cycle_edge) = (vec![], vec![]);

        edges.into_iter().for_each(|edge| {
            let (from, to) = (edge[0], edge[1]);
            if parent[to as usize] != to {
                conflict_edge = edge; // 两个父节点
            } else {
                parent[to as usize] = from;
                if nf.union(from, to) {
                    cycle_edge = edge; // 有环
                }
            }
        });

        if conflict_edge.len() == 0 {
            // [1,2], [2,3], [3,1]
            return cycle_edge; // 没有冲突的边，返回有环的边
        }

        if cycle_edge.len() != 0 {
            println!(
                "conflict_edge: {:?}, cycle_edge: {:?}",
                conflict_edge, cycle_edge
            );
            // [[2,1], [3,1], [4,2], [1,4]]
            // 有冲突同时也有环.
            // 其中conflict_edge[1]是有两个父节点的节点，conflict_edge[0]是其中一个父节点
            return vec![parent[conflict_edge[1] as usize], conflict_edge[1]];
        }
        // [1,2], [1,3], [2,3]
        conflict_edge
    }
}

struct UnionFind(Vec<i32>);

impl UnionFind {
    fn new(n: usize) -> Self {
        Self((0..=n as i32).collect())
    }

    fn find(&mut self, mut x: i32) -> i32 {
        while x != self.0[x as usize] {
            self.0[x as usize] = self.0[self.0[x as usize] as usize];
            x = self.0[x as usize];
        }
        x
    }

    fn union(&mut self, x: i32, y: i32) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.0[root_x as usize] = root_y;
            false
        } else {
            true
        }
    }
}
