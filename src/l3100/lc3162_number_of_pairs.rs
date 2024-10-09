#[allow(unused)]
pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0i32;
    for &x in nums1.iter() {
        for &y in nums2.iter() {
            if x % (y * k) == 0 {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 4, 12];
        let nums2 = vec![2, 4];
        let k = 3;
        let res = 2;
        assert_eq!(number_of_pairs(nums1, nums2, k), res);
    }
}
