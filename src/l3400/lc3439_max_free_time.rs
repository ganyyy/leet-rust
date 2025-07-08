pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    assert_eq!(start_time.len(), end_time.len());

    let length = start_time.len();

    let mut free_time = vec![0; length + 1];
    free_time[0] = start_time[0];
    for i in 1..length {
        free_time[i] = start_time[i] - end_time[i - 1];
    }
    free_time[length] = event_time - end_time.last().unwrap();

    let mut ret = 0;
    let mut cur = 0;
    for (i, &time) in free_time.iter().enumerate() {
        cur += time;
        if i > k as usize {
            cur -= free_time[i - k as usize - 1];
        }
        ret = ret.max(cur);
    }
    ret
}

mod tests {
    #[test]
    fn test_max_free_time() {
        use super::*;

        let ret = max_free_time(10, 1, vec![0, 2, 9], vec![1, 4, 10]);
        println!("ret = {ret}");
    }
}
