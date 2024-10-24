#[allow(unused)]
fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let mut max_i = 0;
    let mut win = 0;
    for i in 1..skills.len() {
        if skills[i] > skills[max_i] {
            // 打擂台，发现新的最大值
            max_i = i;
            win = 0;
        }
        win += 1; // 获胜回合 +1
        if win == k {
            // 首个连续赢下 k 场比赛的人
            break;
        }
    }
    // 如果 k 很大，那么 max_i 就是 skills 最大值的下标，毕竟最大值会一直赢下去
    max_i as _
}
