

struct Solution;

static DIR: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右、下、左、上


#[derive(Eq, Debug)]
struct State {
    x: usize,
    y: usize,
    time: i32,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }    
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}


impl Solution {

    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let mut dis = vec![vec![i32::MAX; m]; n];
        dis[0][0] = 0;

        let mut queue = std::collections::BinaryHeap::new();
        queue.push(State { x: 0, y: 0, time: 0 });

        while let Some(state) = queue.pop() {
            let (x, y, time) = (state.x, state.y, state.time);

            if x == n - 1 && y == m - 1 {
                return time;
            }

            if time > dis[x][y] {
                continue;
            }

            for &(dx, dy) in &DIR {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                    continue;
                }
                let new_time = 1 + time.max(move_time[nx as usize][ny as usize]);
                if new_time >= dis[nx as usize][ny as usize] {
                    continue;
                }
                dis[nx as usize][ny as usize] = new_time;
                queue.push(State { x: nx as usize, y: ny as usize, time: new_time });
            }
        }
        unreachable!()
    }
}