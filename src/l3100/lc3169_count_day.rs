pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    meetings.sort_unstable_by_key(|x| x[0]);

    let mut pre_max = 0;
    let mut ret = 0;

    for meeting in meetings {
        let start = meeting[0];
        let end = meeting[1];

        if pre_max >= start {
            pre_max = pre_max.max(end);
        } else {
            ret += start - pre_max - 1;
            pre_max = end;
        }
    }
    ret + days - pre_max
}
