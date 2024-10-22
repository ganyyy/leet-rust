#[allow(unused)]
fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let mut hash = [0; 24];
    hours.iter().fold(0, |cnt, &hour| {
        let hour = hour % 24;
        let target = if hour == 0 { 0 } else { 24 - hour };
        let cnt = cnt + hash[target as usize];
        hash[hour as usize] += 1;
        cnt
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let hours = vec![12, 12, 30, 24, 24];
        count_complete_day_pairs(hours);
    }
}
