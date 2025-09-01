pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
    // x正序, y逆序
    points.sort_unstable_by_key(|p| (p[0], -p[1]));
    let mut ret = 0;

    let length = points.len();

    for i in 0..length {
        let mut max_y = i32::MIN;
        let y1 = points[i][1];
        for j in i + 1..length {
            let y2 = points[j][1];
            // 这一步很有意思: 如果x相同, 那么只会计算一次;
            // 如果x不同, 那么只会计算递增的y.
            //   因为如果新的y比max_y要小的话, 那么肯定会出现包含
            if y2 <= y1 && y2 > max_y {
                ret += 1;
                max_y = y2;
            }
        }
    }

    ret
}
